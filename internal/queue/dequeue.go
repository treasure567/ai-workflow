package queue

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
	"github.com/rs/zerolog/log"
	"github.com/treasure567/ai-workflow/internal/jobs"
)

func (q *RedisQueue) Dequeue(ctx context.Context, timeout time.Duration) (*jobs.Job, error) {
	result, err := q.client.BRPop(ctx, timeout, QueueKey).Result()
	if err != nil {
		if errors.Is(err, redis.Nil) {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to dequeue: %w", err)
	}

	var job jobs.Job
	if err := json.Unmarshal([]byte(result[1]), &job); err != nil {
		return nil, fmt.Errorf("failed to unmarshal job: %w", err)
	}

	log.Info().Str("job_id", job.ID).Str("type", job.Type).Msg("job received")
	return &job, nil
}

func (q *RedisQueue) PromoteRetryJobs(ctx context.Context) error {
	now := fmt.Sprintf("%d", time.Now().Unix())

	results, err := q.client.ZRangeByScore(ctx, RetryKey, &redis.ZRangeBy{
		Min: "-inf",
		Max: now,
	}).Result()
	if err != nil {
		return fmt.Errorf("failed to query retry queue: %w", err)
	}

	for _, jobData := range results {
		pipe := q.client.Pipeline()
		pipe.LPush(ctx, QueueKey, jobData)
		pipe.ZRem(ctx, RetryKey, jobData)
		if _, err := pipe.Exec(ctx); err != nil {
			log.Error().Err(err).Msg("failed to promote retry job")
			continue
		}
	}

	return nil
}
