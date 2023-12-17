package main

import (
	"fmt"

	"github.com/golang-jwt/jwt/v5"
)

func main() {
	token := jwt.New(jwt.SigningMethodHS256)
	claims, _ := token.Claims.(jwt.MapClaims)
	claims["type"] = "user"
	claims["text"] = "jerome-robert"
	msg, _ := token.SignedString([]byte("xxxxxxx"))
	fmt.Println(msg)
}
