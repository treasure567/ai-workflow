package config

import (
	"fmt"
	"os"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"gopkg.in/yaml.v3"
)

type RedisConfig struct {
	Host     string `yaml:"host"`
	Port     string `yaml:"port"`
	Password string `yaml:"password"`
}

func (r RedisConfig) Addr() string {
	return r.Host + ":" + r.Port
}

type SMTPConfig struct {
	Host     string `yaml:"host"`
	Port     int    `yaml:"port"`
	User     string `yaml:"user"`
	Password string `yaml:"password"`
}

type Config struct {
	Redis             RedisConfig `yaml:"redis"`
	SMTP              SMTPConfig  `yaml:"smtp"`
	WorkerConcurrency int         `yaml:"worker_concurrency"`
	MaxRetries        int         `yaml:"max_retries"`
	APIToken          string      `yaml:"api_token"`
}

func Load(path string) (*Config, error) {
	zerolog.TimeFieldFormat = zerolog.TimeFormatUnix
	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stdout})

	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read config file: %w", err)
	}

	cfg := &Config{
		Redis: RedisConfig{
			Host: "localhost",
			Port: "6379",
		},
		SMTP: SMTPConfig{
			Port: 587,
		},
		WorkerConcurrency: 5,
		MaxRetries:        3,
	}

	if err := yaml.Unmarshal(data, cfg); err != nil {
		return nil, fmt.Errorf("failed to parse config file: %w", err)
	}

	if cfg.WorkerConcurrency < 1 {
		return nil, fmt.Errorf("worker_concurrency must be at least 1")
	}

	if cfg.MaxRetries < 0 {
		return nil, fmt.Errorf("max_retries must be non-negative")
	}

	return cfg, nil
}
