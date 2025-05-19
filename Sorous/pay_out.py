import uiautomator2 as u2
import time


def pay_out(upi_id,amt):
    d = u2.connect()
    d.settings['wait_timeout'] = 5.0
    #d(resourceId="com.ktkbank.digitalrupee:id/iv_dashboard_send_rupee").click()
    #d(resourceId="com.ktkbank.digitalrupee:id/bnv_history").click()
    #d.app_start("com.android.chrome")

    d.app_start("com.fampay.in")
    d(text="Pay UPI ID or number").click()

    d(resourceId="com.fampay.in:id/upi_address_input").click()

    d(resourceId="com.fampay.in:id/upi_address_input").set_text(upi_id)
    try:
        d(resourceId="com.fampay.in:id/verified_details_subtitle").click()
    except:
        return {"status": "error", "message": "UPI ID not found"} 
    if amt<1:
        return {"status": "error", "message": "Amount should be greater than 1"}
    amt = str(amt)
    for a in amt:
        if a == ".":
            d(resourceId="com.fampay.in:id/fam_keyboard_decimal").click()
        else:
            d(resourceId="com.fampay.in:id/fam_keyboard_"+a).click()
    d(resourceId="com.fampay.in:id/pay_button_wrapper").click()
    d(resourceId="com.android.systemui:id/button_bar").click()
    d(resourceId="com.android.systemui:id/lockPassword").set_text("1235789")
    d(resourceId="com.android.systemui:id/footerRightButton").click()
    time.sleep(10)
    d.screenshot("screenshot.png")
    return {"status": "success", "message": "Payment successful", "screenshot": "screenshot.png"}
