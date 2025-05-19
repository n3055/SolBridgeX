import hmac
import hashlib
from fastapi import FastAPI, Request, HTTPException
from pydantic import BaseModel
from pay_out import pay_out
import time

app = FastAPI()

SECRET_KEY = b"f93e63dbe7a75f42c58f2c1d527b3d9a67N3055RA"

TIME_WINDOW = 300

def make_payment(upi_id: str, amt: float):
    return pay_out(upi_id, amt)

class PaymentRequest(BaseModel):
    upi_id: str
    amt: float
    timestamp: int

def generate_signature(timestamp: int, upi_id: str, amt: float) -> str:
    message = f"{timestamp}:{upi_id}:{amt}".encode()
    signature = hmac.new(SECRET_KEY, message, hashlib.sha256).hexdigest()
    return signature

@app.post("/secure-payment")
async def secure_payment(request: Request, payment: PaymentRequest):
    client_signature = request.headers.get('X-Signature')
    if not client_signature:
        raise HTTPException(status_code=400, detail="Signature missing")

    current_timestamp = int(time.time())
    if abs(current_timestamp - payment.timestamp) > TIME_WINDOW:
        raise HTTPException(status_code=400, detail="Request timestamp expired")

    expected_signature = generate_signature(payment.timestamp, payment.upi_id, payment.amt)

    if not hmac.compare_digest(client_signature, expected_signature):
        raise HTTPException(status_code=401, detail="Invalid Signature")

    result = make_payment(payment.upi_id, payment.amt)
    return result