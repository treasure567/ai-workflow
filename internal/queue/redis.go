package queue

import (
	"context"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
	"github.com/treasure567/ai-workflow/pkg/config"
)

const (
	QueueKey = "jobs:queue"
	RetryKey = "jobs:retry"
	DeadKey  = "jobs:dead"
)

type RedisQueue struct {
	client *redis.Client
}

func NewRedisQueue(cfg config.RedisConfig) (*RedisQueue, error) {
	client := redis.NewClient(&redis.Options{
		Addr:     cfg.Addr(),
		Password: cfg.Password,
		DB:       0,
	})

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := client.Ping(ctx).Err(); err != nil {
		return nil, fmt.Errorf("redis connection failed: %w", err)
	}

	return &RedisQueue{client: client}, nil
}

func (q *RedisQueue) Close() error {
	return q.client.Close()
}
