import urllib.request
from concurrent.futures import ThreadPoolExecutor
import time

loop = 500
concurrency = 5
endpoints = {"rust": "http://127.0.0.1:7878"}


# 並列で実行するHTTPのリクエスト関数
def request(i, url):
    try:
        response = urllib.request.urlopen(url)
        body = response.read().decode("utf-8")

    except Exception as e:
        print(e)


if __name__ == "__main__":
    for k, v in endpoints.items():
        print("request to {} server".format(k))
        start = time.time()
        with ThreadPoolExecutor(max_workers=concurrency) as executor:
            for i in range(loop):
                executor.submit(request, i=i, url=v)
        print("{} elapsed: {}".format(k, time.time() - start))
