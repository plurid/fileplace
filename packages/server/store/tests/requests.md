# Store

curl -v -F 'files[]=@file' http://127.0.0.1:56165/store

curl -v -F 'files[]=@file.png' http://127.0.0.1:56165/store?owner=one&place=two&name="a_file.png"

curl -v -F 'files[]=@file.png' http://127.0.0.1:56165/store?place=two&name="a_file.png"


# Metadata

curl -v http://127.0.0.1:56165/metadata?owner=one&place=two&name="a_file.png"

curl -v http://127.0.0.1:56165/metadata?place=two&name="a_file.png"


# Places

curl -v http://127.0.0.1:56165/places


# Exists

curl -v http://127.0.0.1:56165/exists?owner=one&place=two&name="a_file.png"

curl -v http://127.0.0.1:56165/exists?place=two&name="a_file.png"


# Remove

curl -v http://127.0.0.1:56165/remove?owner=one&place=two&name="a_file.png"

curl -v http://127.0.0.1:56165/remove?place=two&name="a_file.png"


# All

curl -v http://127.0.0.1:56165/all?owner=one&place=two

curl -v http://127.0.0.1:56165/all?place=two


# Get

curl -v http://127.0.0.1:56165/get?owner=one&place=two&name="a_file.png" -o a_file.png

curl -v http://127.0.0.1:56165/get?place=two&name="a_file.png" -o a_file.png
