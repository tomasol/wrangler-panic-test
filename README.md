# worker-panic-test

```sh
wrangper publish
hey -n 1000 $URL
# [200] 100% of the time
curl $URL/crash
hey -n 1000 $URL
# [500] around 1-2% of requests within 1-2 minutes

# for better effect:
hey -n 1000 $URL/crash
```
