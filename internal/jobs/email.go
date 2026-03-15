package jobs

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"path"
	"time"

	"github.com/treasure567/ai-workflow/pkg/config"
	"gopkg.in/gomail.v2"
)

type EmailPayload struct {
	To          string   `json:"to"`
	Subject     string   `json:"subject"`
	Body        string   `json:"body"`
	From        string   `json:"from"`
	Attachments []string `json:"attachments"`
}

func ProcessEmail(cfg *config.Config, data json.RawMessage) error {
	var payload EmailPayload
	if err := json.Unmarshal(data, &payload); err != nil {
		return fmt.Errorf("invalid email payload: %w", err)
	}

	if payload.To == "" || payload.Subject == "" || payload.Body == "" {
		return fmt.Errorf("email requires to, subject, and body fields")
	}

	if payload.From == "" {
		payload.From = cfg.SMTP.User
	}

	m := gomail.NewMessage()
	m.SetHeader("From", payload.From)
	m.SetHeader("To", payload.To)
	m.SetHeader("Subject", payload.Subject)
	m.SetBody("text/html", payload.Body)

	var tempFiles []string
	defer func() {
		for _, f := range tempFiles {
			os.Remove(f)
		}
	}()

	for _, fileURL := range payload.Attachments {
		tempPath, err := downloadAttachment(fileURL)
		if err != nil {
			return fmt.Errorf("failed to download attachment %s: %w", fileURL, err)
		}
		tempFiles = append(tempFiles, tempPath)
		m.Attach(tempPath)
	}

	d := gomail.NewDialer(cfg.SMTP.Host, cfg.SMTP.Port, cfg.SMTP.User, cfg.SMTP.Password)
	if err := d.DialAndSend(m); err != nil {
		return fmt.Errorf("failed to send email: %w", err)
	}

	return nil
}

func downloadAttachment(fileURL string) (string, error) {
	client := &http.Client{Timeout: 30 * time.Second}

	resp, err := client.Get(fileURL)
	if err != nil {
		return "", fmt.Errorf("failed to fetch file: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return "", fmt.Errorf("file download returned status %d", resp.StatusCode)
	}

	filename := path.Base(fileURL)
	if filename == "" || filename == "." || filename == "/" {
		filename = "attachment"
	}

	tmpFile, err := os.CreateTemp("", "email-attachment-*-"+filename)
	if err != nil {
		return "", fmt.Errorf("failed to create temp file: %w", err)
	}
	defer tmpFile.Close()

	if _, err := io.Copy(tmpFile, resp.Body); err != nil {
		os.Remove(tmpFile.Name())
		return "", fmt.Errorf("failed to write attachment: %w", err)
	}

	return tmpFile.Name(), nil
}
