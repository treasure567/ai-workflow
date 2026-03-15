package jobs

import (
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
)

const (
	TypeWebhook = "webhook"
	TypeEmail   = "email"
)

var BackoffSchedule = []time.Duration{
	10 * time.Second,
	30 * time.Second,
	2 * time.Minute,
}

type Job struct {
	ID        string          `json:"id"`
	Type      string          `json:"type"`
	Data      json.RawMessage `json:"data"`
	Retries   int             `json:"retries"`
	CreatedAt time.Time       `json:"created_at"`
}

func NewJob(jobType string, data json.RawMessage) *Job {
	return &Job{
		ID:        uuid.New().String(),
		Type:      jobType,
		Data:      data,
		Retries:   0,
		CreatedAt: time.Now().UTC(),
	}
}

func (j *Job) Validate() error {
	switch j.Type {
	case TypeWebhook, TypeEmail:
	default:
		return fmt.Errorf("unsupported job type: %s", j.Type)
	}

	if len(j.Data) == 0 {
		return fmt.Errorf("job data is required")
	}

	if !json.Valid(j.Data) {
		return fmt.Errorf("job data is not valid JSON")
	}

	return nil
}
