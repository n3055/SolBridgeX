import uiautomator2 as u2
import time


def pay_out(upi_id,amt,pin):
    d = u2.connect()
    d.settings['wait_timeout'] = 3.0
    d.app_start("com.google.android.apps.nbu.paisa.user")

    d.xpath('//*[contains(@content-desc,"Pay anyone on UPI, Pay friends and merchants, Pay by name or phone number")]').click()

    d.xpath('//*[contains(@hint,"Pay anyone on UPI")]').set_text(upi_id)
    try:
        d.xpath('//*[contains(@content-desc,"{}")]'.format(upi_id)).click()
    except:
        return {"status": "error", "message": "UPI ID not found"}
    if amt<1:
        return {"status": "error", "message": "Amount should be greater than 1"}
    d.xpath('//*[normalize-space(@content-desc)="Pay"]').click()
    d.xpath('//*[contains(@hint,"Amount field\n0")]').set_text(str(amt))
    d.xpath('//*[normalize-space(@bounds)="[594,1317][692,1415]"]').click()
    d.xpath('//*[normalize-space(@bounds)="[42,1374][678,1458]"]').click()
    for p in pin:
        d.xpath('//*[normalize-space(@text)="{}"]'.format(p)).click()
    d.xpath('//*[normalize-space(@bounds)="[480,1420][720,1487]"]').click()
    return {"status": "success", "message": "Payment successful", "screenshot": "screenshot.png"}

#print(pay_out("8310431497@fam", 5))

