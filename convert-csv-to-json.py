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
    sums  = 0
    length = 0
    index = 1
    for i in range(1, sheet.nrows):
        item = sheet.row_values(i)
        print("acmount: ", item)
        account = item[index]
        # print("account: ", account, ", amount: ", float(item[2]))
        try:
            amount_str = re.findall(r'\d+', item[index + 1])[0]
            amount = float(amount_str)
        # amount = float(item[1])
        except:
            print("acmount: ", item[index + 1])
            amount = item[index + 1]
            print("acmount: ", amount)
        reward = {
            "account": account,
            "amount": amount
        }
        rewards.append(reward)
        length += 1
        sums += amount
    print(rewards)
    print("sum: ", sums)
    print("length: ", length)

    dump_json_to_file(json_path, rewards)


if __name__ == "__main__":
    excel_path = "/home/bifrost/jdeng/bifrost-xt/2021-02-26/2.8 DeFiGo ama 活动发奖.xlsx"
    convert_excel_to_json(excel_path, "/home/bifrost/jdeng/bifrost-xt/2021-02-26/2.8 DeFiGo ama 活动发奖.xlsx.json")

'''
Sumary

3.21 早鸟节点奖励发放记录
发放数量: 1566.5
参与人数：36

3.25 海外 KSM 空投
发放数量：120.000000172997
参与人数：1119

总发放数量：1686.500000172997
'''