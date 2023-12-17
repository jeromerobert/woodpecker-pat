TODO: get PICUS_WOODPECKER_TOKEN from sqlite3 db
sqlite3 woodpecker.sqlite 'select user_hash from users where user_login="jerome-robert"'
https://docs.rs/jwt/latest/jwt/
https://github.com/woodpecker-ci/woodpecker/blob/main/shared/token/token.go#L114
https://github.com/windsource/picus/blob/main/src/main.rs#L35

```
github.com/golang-jwt/jwt/v5 v5.2.0
```
```go
func (t *Token) SignExpires(secret string, exp int64) (string, error) {
	token := jwt.New(jwt.SigningMethodHS256)
	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		return "", fmt.Errorf("token claim is not a MapClaims")
	}
	claims["type"] = t.Kind
	claims["text"] = t.Text
	if exp > 0 {
		claims["exp"] = float64(exp)
	}
	return token.SignedString([]byte(secret))
}
```