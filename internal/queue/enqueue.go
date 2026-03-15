package queue

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
	"github.com/rs/zerolog/log"
	"github.com/treasure567/ai-workflow/internal/jobs"
)

func (q *RedisQueue) Enqueue(ctx context.Context, job *jobs.Job) error {
	data, err := json.Marshal(job)
	if err != nil {
		return fmt.Errorf("failed to marshal job: %w", err)
	}

	if err := q.client.LPush(ctx, QueueKey, data).Err(); err != nil {
		return fmt.Errorf("failed to enqueue job: %w", err)
	}

	log.Info().Str("job_id", job.ID).Str("type", job.Type).Msg("job queued")
	return nil
}

func (q *RedisQueue) EnqueueRetry(ctx context.Context, job *jobs.Job, delay time.Duration) error {
	data, err := json.Marshal(job)
	if err != nil {
		return fmt.Errorf("failed to marshal job for retry: %w", err)
	}

	score := float64(time.Now().Add(delay).Unix())
	if err := q.client.ZAdd(ctx, RetryKey, redis.Z{Score: score, Member: data}).Err(); err != nil {
		return fmt.Errorf("failed to schedule retry: %w", err)
	}

	log.Info().Str("job_id", job.ID).Int("retry", job.Retries).Dur("delay", delay).Msg("retry scheduled")
	return nil
}

func (q *RedisQueue) MoveToDeadQueue(ctx context.Context, job *jobs.Job) error {
	data, err := json.Marshal(job)
	if err != nil {
		return fmt.Errorf("failed to marshal job for dead queue: %w", err)
	}

	if err := q.client.LPush(ctx, DeadKey, data).Err(); err != nil {
		return fmt.Errorf("failed to move to dead queue: %w", err)
	}

	log.Info().Str("job_id", job.ID).Msg("job moved to dead queue")
	return nil
}
