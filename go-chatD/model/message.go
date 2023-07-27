package model

type Message struct {
	Id             string
	ConversationId string
	Content        string
	SentTime       int64
	FromUserId     string
}
