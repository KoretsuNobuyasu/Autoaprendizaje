#! /usr/bin/env python3
#-*- coding:utf-8 -*-
#__author__ == nobu
#__date__ == 2018/10/08
#__version__ == 1.0.0

# import
import json, requests, sys
# class


EXIT_SUCCESS = 0
EXIT_FAILURE = 1
#openweathermap.org API key
APPID='a584bd2a6688a2c48c50b3775c6d691b'

#openweathermap.org の　APIからJSONデータをダウンロード
url = 'http://api.openweathermap.org/data/2.5/forecast/daily?q={}&cnt=3&appid={}'.format(location, APPID)



def main():

    if len(sys.argv) < 2:
        print('Usage: weather.py location')
        sys.exit()
    location = ' '.join(sys.argv[1:])
    get_info()

    return  EXIT_SUCCESS


def get_info():
    response = requests.get(url)
    response.raise_for_status()


if __name__ == '__main__':

    main()