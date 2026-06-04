# domectrl
#### A simple GUI utility to control the telescope domes at Pine Mountain Observatory
---


## 1.  Overview
domectrl provides a simpler way to interface with PMO's dome controller.<br/>
Under the hood, it is simply a serial connection sending and receiving single-byte streams.<br/>
### 1.1 Main Page
the main page of domectrl shows the serial port connection status, raise/lower controls, the controller response, and the abort button which forcefully ceases all data output in the event of a failure
![main page screenshot](/media/main.png)
### 1.2 Settings Page
domectrl supports robust configuration through this page. <br />
configurable values are:

|Value|Description|Default|
|-----|-----------|-------|
|Serial Port|Port to send data on|COM4|
|Baud rate|Baud rate of serial connection|9600|
|Send interval|Interval (ms) between bytes in data stream|100|
|Auto-reconnect|Reconnect if connection drops|yes|
|Reconnect interval|Interval (ms) of connection retry|3000|
|Commands|Bytes to send for raising/lowering sides|a,b,A,B|
|Config directory|Where to look for config files|User's home dir (OS-specific)|
<br/>
![settings page screenshot](/media/settings.png)
### 1.3 Help Page (?)
domectrl's help page shows some useful information about how to use the program, most of which is an abridged version of this README




## Acknowledgements
- Dr. Scott Fisher and Alton Luken at Pine Mountain


