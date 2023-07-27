package service

import (
	"github.com/go-redis/redis"
	"go-chatD/model"
)

type ConversationService struct {
	conversations map[string]*model.Conversation
	client2convs  map[*model.Client][]*model.Conversation
	redis         *redis.Client
}

func NewConversationService(redisClient *redis.Client) *ConversationService {
	return &ConversationService{
		conversations: make(map[string]*model.Conversation),
		client2convs:  make(map[*model.Client][]*model.Conversation),
		redis:         redisClient,
	}
}

// CreateConversation creates a new conversation with provided id
func (service *ConversationService) CreateConversation(id string) (string, error) {
	conversation := &model.Conversation{id, make([]*model.Client, 1)}
	service.conversations[id] = conversation
	// put conversation into the redis cache
	command := service.redis.HSet("conversations", id, conversation)
	if command.Err() != nil {
		return id, command.Err()
	}
	return id, nil
}
