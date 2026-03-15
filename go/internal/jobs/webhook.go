package jobs

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

type WebhookPayload struct {
	URL     string            `json:"url"`
	Method  string            `json:"method"`
	Headers map[string]string `json:"headers"`
	Body    json.RawMessage   `json:"body"`
}

func ProcessWebhook(ctx context.Context, data json.RawMessage) error {
	var payload WebhookPayload
	if err := json.Unmarshal(data, &payload); err != nil {
		return fmt.Errorf("invalid webhook payload: %w", err)
	}

	if payload.URL == "" {
		return fmt.Errorf("webhook url is required")
	}

	if payload.Method == "" {
		payload.Method = http.MethodPost
	}

	client := &http.Client{Timeout: 30 * time.Second}

	req, err := http.NewRequestWithContext(ctx, payload.Method, payload.URL, bytes.NewReader(payload.Body))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	for k, v := range payload.Headers {
		req.Header.Set(k, v)
	}

	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("webhook request failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("webhook returned status %d", resp.StatusCode)
	}

	return nil
}
