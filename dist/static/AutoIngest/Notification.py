def send_success_message_technology(package_name,title):
    headers = {
        'Content-Type': 'application/json'
    }
    body_list = []
    aspera_notification = payload_header_builder('Aspera Notification: Success')  
    aspera_notification["color"] = "good"
    aspera_notification["weight"] = "bolder"
    body_list.append(aspera_notification)
    fact_dict = {
        "Package Name:": package_name,
        "Title:": title
    }
    body_list.append(payload_fact_build(fact_dict))

    payload = __PAYLOAD__
    payload["body"] = body_list
    _ = requests.post(url=os.environ.get('DEV_WEBHOOK_URL'), headers=headers, data=json.dumps(payload))