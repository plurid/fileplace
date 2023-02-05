# Store

curl -v -F 'files[]=@file' http://127.0.0.1:56165/store

curl -v -F 'files[]=@file.png' http://127.0.0.1:56165/store?owner=one&place=two&name="a_file.png"

curl -v -F 'files[]=@file.png' http://127.0.0.1:56165/store?place=two&name="a_file.png"


# Exists

curl -v http://127.0.0.1:56165/exists?owner=one&place=two&name="a_file.png"

curl -v http://127.0.0.1:56165/exists?place=two&name="a_file.png"


# Remove

curl -v http://127.0.0.1:56165/remove?owner=one&place=two&name="a_file.png"

curl -v http://127.0.0.1:56165/remove?place=two&name="a_file.png"
