import hmac
import hashlib
import time
import requests

SECRET_KEY = b"f93e63dbe7a75f42c58f2c1d527b3d9a67N3055RA"

API_URL = "http://192.168.29.72:8000/secure-payment"

def generate_signature(upi_id, amt, timestamp):
    message = f"{timestamp}:{upi_id}:{amt}".encode()
    return hmac.new(SECRET_KEY, message, hashlib.sha256).hexdigest()

upi_id = "nk7387631@okhdfcbank"
amt = 1.0  
timestamp = int(time.time()) 

signature = generate_signature(upi_id, amt, timestamp)

headers = {
    "X-Signature": signature 
}

data = {
    "upi_id": upi_id,
    "amt": amt,
    "timestamp": timestamp
}

response = requests.post(API_URL, json=data, headers=headers)

if response.status_code == 200:
    print("Payment Success:", response.json())
else:
    print(f"Error {response.status_code}: {response.text}")
