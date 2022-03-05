# Weather

Gives you your weather based on your IP Address.

## Example

```
+---------------+---------+------+------------+----------+----------+
| city          | country | temp | feels_like | temp_min | temp_max |
+---------------+---------+------+------------+----------+----------+
| New York City | US      | 36   | 31         | 28       | 41       |
+---------------+---------+------+------------+----------+----------+
```

To run this app, you'll need your own API key from openweathermap, and
then you can run the app by passing your API key as an environment
variable called `APP_ID` on the command line, like `APP_ID=1234 cargo run` to run the app in development mode.
