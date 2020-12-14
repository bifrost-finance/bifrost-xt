#!/usr/bin/env python3

import json
import re
import xlrd

def read_json_file(path):
    f = open(path, 'r')
    data = f.read()
    return json.loads(data)

def dump_json_to_file(path, content):
    f = open(path, 'w')
    data = json.dumps(content)
    f.write(data)
    f.truncate()
    f.close()

def convert_excel_to_json(excel_path, json_path=None):
    wb = xlrd.open_workbook(excel_path)
    sheet = wb.sheet_by_index(0)

    rewards = []
    for i in range(1, sheet.nrows):
        item = sheet.row_values(i)
        account = item[1]
        amount = item[2]
        reward = {
            "account": account,
            "amount": amount
        }
        rewards.append(reward)
    print(rewards)
    dump_json_to_file(json_path, rewards)


if __name__ == "__main__":
    excel_path = "/home/bifrost/jdeng/bifrost-xt/2020-12-14/defigo 社区AMA发奖记录.xlsx"
    convert_excel_to_json(excel_path, "2020-12-14/defigo 社区AMA发奖记录.json")