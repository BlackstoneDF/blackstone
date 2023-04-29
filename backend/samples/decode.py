# A script to decode templates, kind of sucks right now because you need to get the template stuff only

#'{"author":"You name","name":"Cool name","version":1,"code":"<KEEP THIS PART>"}'
# Only keep the part where is says <KEEP THIS PART>

import base64
import gzip

input_string = input("Data: \n")

decoded_bytes = base64.b64decode(input_string)

uncompressed_bytes = gzip.decompress(decoded_bytes)

decoded_string = uncompressed_bytes.decode('utf-8')

print(decoded_string)
