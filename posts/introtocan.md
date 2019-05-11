<!--
Introduction to CAN
2019-5-10 12:00
wiki,electronics,CAN
CAN (controller area network) is a system for allowing many devices to communicate in electrically noisy environments in real time.
-->

# Introduction to CAN

### Table of Contents
i. [CAN Overview](#overview)  
ii. [CAN Electronics](#electronics)  
iii. [CAN RoboRIO Programming](#roborio_code)  
iv. [CAN Teensy Programming](#teensy_code)  
v. [CAN Debugging](#debugging)

<a name="overview"></a>
## I. CAN Overview

CAN (controller area network) is a system for allowing many devices to communicate in electrically noisy environments in real time. Devices that we use that use CAN in FRC are the RoboRIO, the PCM, the PDP, and Talon SRX motor controllers, in addition to the Teensy CAN boards. A device connected to the CAN bus is occasionally referred to as a CAN node or a node.

### I.I Why CAN?

We found that it was impractical to connect all of the sensors we wanted to have on our robot directly to the roborio. We also had many applications where it would be nicer to have a smaller, more real-time system that can reliably process data packets from sensors such as a LIDAR or handle more interrupts from sensors such as encoders than the Roborio can handle. The common solution would be to use a protocol such as I2C or serial, but both of these have more significant downsides than CAN. I2C has problems handling cables longer than about 1 meter, and works sub-optimally in high electrical noise environments. RS-232 serial may be used for higher signal robustness, however it is harder to interface directly to sensors. The roborio also has a limited number of serial ports available, and this method also creates a lot more complexity in wire management. 

CAN bus uses differential signaling on two data pins. Any nodes can send data (multimaster). Electrical noise can still alter the CAN signals, but because the wires are twisted, they receive the same alterations. In this way, the voltages stay the same relative to each other, and the difference remains constant.

<a name="electronics"></a>
## II. CAN Electronics

### General CAN Electronics

The CAN bus is the pair of green and yellow wires that connect to each node. The wires are connected to each node in parallel, and are twisted together in order to aid in noise reduction. One end of the wires must be at the RoboRIO, and in most cases the other end should be at the PDP. While it is possible to have the wires branch (which is referred to as the "star topology"), this is not recommended and adds noise.

One of the key differences between CAN and many protocols is that CAN uses a differential voltage. Instead of representing a signal as the difference between ground and some other wire, CAN represents a signal by two wires being at the same voltage or at different voltage.

![Image of CAN signal](canbus_waveform.jpg)

When the voltages are different, some device is transmitting a 0. When the voltages are the same, some device is transmitting a 1.

### Teensy CAN Electronics

Most CAN connection systems have two components: a CAN controller and a CAN interface. The CAN controller converts the signals from a common microcontroller protocol (SPI and serial are popular) to the CAN protocol. The CAN interface then converts the CAN protocol signals (which are sent as voltages between 0-5V) to the CAN differential voltage. The Teensy 3.1, 3.2, 3.5, and 3.6 each have a CAN controller built in, and only require an external CAN interface. The MCP2561 chip is that CAN interface.

![Image of CAN chips](CAN_microchips.png)

Here is a diagram of the connections between the relevant chips. TX/RX and CAN TX/CAN RX signals are relative to ground. Note that simultaneous TX and RX is not possible (as a node is either reading or writing to the CAN bus, not both); this is drawn here only for the purpose of showing that there can be signals on those wires.

The Teensy to MCP2561 connections should be based on the pinouts available at [pjrc.com](https://www.pjrc.com/teensy/teensy32_front_pinout.png)

#### MCP2561 Wiring

The MCP2561s we use are in DIP-8 packages. Here is a pinout for one of these:

![MCP2561 Pinout](mcp2561.png)

The TXD and RXD pins are for commmunicating with the CAN controller (TXD to TX and RXD to RX). VSS and VDD are ground and power (5V) respectively. CANH and CANL connect directly to the CAN bus high (yellow) and low (green). If the MCP2561 is at the end of the CAN bus (which should not happen for typical FRC wiring), a ~120 Ohm resistor should be placed between CANH and CANL. Rs is the slope control pin. This allows the speed at which the MCP2561 transistions between sending 1s and 0s to change. The speed decreases as the resistance between this pin and ground increases. If at 5V, write is disabled. As the RoboRIO CAN bus operates at high speed mode, this pin needs to be grounded to have maximum speed.

<a name="roborio_code"></a>
## III. RoboRIO Programming

### JNI Documentation

##### Purpose of JNI

The general purpose of JNI is because it's difficult to communicate from Java directly to the FPGA. When we send commands through the CPU the messages go to the FPGA, which send data to PWN, DIO, CAN, etc. Java cannot communicate directly to the FPGA, but because C++ is compiled natively, we use JNI to send Java data through C++ to the FPGA.

##### How JNI works in the code base

###### Writing to CAN

Take a look at the code to write data to CAN, and break it down.

```
public void writeSafely(byte[] data) {
		ByteBuffer canData = ByteBuffer.allocateDirect(8);
		canData.put(data);
		CANJNI.FRCNetCommCANSessionMuxSendMessage(messageID, canData, CANJNI.CAN_SEND_PERIOD_NO_REPEAT);
}
```
The function takes in a byte array of data, and with that...

1. The first line, `ByteBuffer canData = ByteBuffer.allocateDirect(8);`, allows the message that you are sending to take up 64 bits — it's directly allocating 8 bytes: `ByteBuffer.allocateDirect(8);`.

2. The second line, `canData.put(data);`, copies the data from the byte array into the current byte buffer (very, very necessary).

3. The third line, `CANJNI.FRCNetCommCANSessionMuxSendMessage(messageID, canData, CANJNI.CAN_SEND_PERIOD_NO_REPEAT);`, calls a function that takes in 3 variables — the `messageID`, the `canData`, and `CANJNI.CAN_SEND_PERIOD_NO_REPEAT`.
	* The `messageID` allows the reader of the message to know what they should with the message.
	* The `canData` is the actual data to send.
	* The `CANJNI.CAN_SEND_PERIOD_NO_REPEAT`, or at least that slot, is the frequency to resend the messages. Because the variable `CANJNI.CAN_SEND_PERIOD_NO_REPEAT` itself is set to 0, it only sends once. Generally, we leave this value along.

###### Reading Byte Buffers from CAN

Again, let's break down the method of reading byte buffers from CAN.

```
protected ByteBuffer readBuffer() throws CANMessageUnavailableException {
		IntBuffer idBuffer = ByteBuffer.allocateDirect(4).asIntBuffer();
		idBuffer.clear();
		idBuffer.put(0, Integer.reverseBytes(messageID));
		ByteBuffer timestamp = ByteBuffer.allocate(4);
		try {
			return CANJNI.FRCNetCommCANSessionMuxReceiveMessage(idBuffer, 0x1fffffff, timestamp);
		}
		catch (CANMessageNotFoundException e) {
			throw new CANMessageUnavailableException("Unable to read CAN device " + getName() + " with ID 0x" + Integer.toHexString(messageID), e);
		}
	}
```
First of all, notice the declaration of the function. It has the ability to throw an error if there isn't a CAN message. Also, it's protected because, to quote Carter T, "byte buffers are annoying".

1. The first line, `IntBuffer idBuffer = ByteBuffer.allocateDirect(4).asIntBuffer();`, allocates 4 bytes for the byte buffer as an int buffer. It is required to be an int buffer.
2. It then clears any previous data in the id buffer in `idBuffer.clear();`.
3. This next line, `idBuffer.put(0, Integer.reverseBytes(messageID));`, is kinda complicated. Let's break it down. It will read the message ID from the file itself... backwards. It reads it backwards because in a string of 0s and 1s, it can read forwards-backwards and vice versa. Because teensies and the Roborio represent bytes in different directions, you need to reverse the direction of the bytes.
4. It then leaves 4 bytes for a timestamp in `ByteBuffer timestamp = ByteBuffer.allocate(4);`.
5. Next, it attempts to read from CAN the data, with the declaration `return CANJNI.FRCNetCommCANSessionMuxReceiveMessage(idBuffer, 0x1fffffff, timestamp);`.
		* The `idBuffer` is the messageID to read from.
	* The `0x1fffffff` takes the 32 bytes that are recieved, and it makes the first 3 bits 0s, as those are just being used by interals. You can change this constant to change the modifications upon the reading, but please don't.
	* It then gives you the `timestamp`, in case you accidentally don't read the most recent file.
6. Finally, if a `CANMessageNotFoundException`, it returns an error which says the name of the CAN messageID that could not be read.

###### Reading in CAN

Let's break it down!

```
public byte[] read() throws CANMessageUnavailableException {
		ByteBuffer dataBuffer = readBuffer();
		if (dataBuffer == null) {
			return null;
		}
		if (dataBuffer.remaining() <= 0) {
			return null;
		}
		dataBuffer.rewind();
		byte[] data = new byte[dataBuffer.remaining()];
		for (int i = 0; i < dataBuffer.remaining(); i++) {
			data[i] = dataBuffer.get(i);
		}
		return data;
	}
```

Pay attention to ho this could throw an error, if no data is available to read.

1. First, it runs the `readBuffer();` function to take data from the can buffer.
2. Then, if there is nothing in the buffer or the size is 0, it returns no and thereby cancels the function.
3. Then, it resets the buffer so that it is able to be read again at `dataBuffer.rewind();`.
4. Then, it creates a new byte with the size of dataBuffer.
5. It then copies the data from `dataBuffer` to the data byte.
6. It then returns that data.

<a name="teensy_code"></a>
## IV. Teensy Programming

On the Teensy side, the lowest level code is the [FlexCAN](https://github.com/teachop/FlexCAN_Library) library. This code provides a variety of functions, the most significant being a write and read function, each of which take in a special `CAN_message_t` struct.

The TeensyCANBase library provides a simpler interface to FlexCAN. It uses a callback system for reading, which can be interacted with either through setting function pointers for certain CAN message IDs via `CAN_add_id` or by passing AbstractTeensyCAN objects to `CAN_add`. It is important to call `CAN_update` frequently to run all of the callbacks. Writing is done through the `CAN_write` function.

Important to note is the Arduino/Teensyduino version compatibility of the TeensyCANBase library. Due to recent changes in FlexCAN, there is an compatibility issue with the way FlexCAN initiates CAN and the way TeensyCANBase expects it to. As such, the current version of TeensyCANBase only works with Arduino 1.6.15 and Teensyduino 1.34.

<a name="debugging"></a>
## V. CAN Debugging

Given the importance of the CAN bus to power distribution, pneumatics control, motor control, and sensing, it is importance that CAN bus issues be resolved as quickly as possible. Here are some of the signs to look for in debugging the CAN bus.

The PDP, PCM, and Talons all have indicator lights. The lights on these devices blink yellow while the robot is disabled. They will blink either briefly or consistently red when there are issues with a given device's connection to the RoboRIO. The PDP and PCM turn green when the robot is enabled. Talons turn solid yellow.The `CTRE CAN Recieve Timeout` error that appears on the Driver Station is a manifestation of a CAN error related to the PDP, PCM, or a Talon.

The most common issue is a disconnected wire. The CAN bus tends to be long and pass through almost every part of the robot, which makes it easy for a wire to be disconnected. If some of the PDP, PCM, and Talons begin flashing red, then it is probable that there is a disconnected wire. The break in the wire is between the last device with the good indicator light pattern and the first device with the bad indicator pattern. Note that the light pattern is an indicator for connection with the RoboRIO, so starting at the device that is reporting a CAN bus connection error and moving towards the RoboRIO is a good strategy.

Another common issue is the `CAN data outdated` error. This error is specific to Teensy CAN boards. The cause of this error is a Teensy board being disconnected, unpowered, or incorrectly programmed. The entire bus being disconnected case is probably fairly unlikely without the error also appearing for the PDP. It is also possible for the Teensy to be disconnected from the MCP2561. This is unlikely on a competition robot, but could occur on a breadboard. This can be checked fairly easily, and also manifests as 8 second delays when `CAN_write` is called. The unpowered issue can be subtle, as there are multiple failure points. If possible, try connecting a USB cable to the Teensy and check if the error persists. This is also a good opportunity to reprogram the Teensy so you are sure it is using the correct CAN IDs. If the board is unpowered, measure voltages at various places to try to isolate the issue. If dealing with a CAN ID issue, be sure to keep in mind that the `CAN data outdated` error prints CAN IDs in base 10, while IDs are typically written in base 16.

Fairly uncommon on competition circuits, but possible, are incorrectly connected wires. Only if every PDP, PCM, and Talon is flashing red is this the error. Look for the wires connected backwards at some point on the CAN bus.

Finally, there is the possibility of Teensy CAN board chip failure. Both the Teensy and the MCP2561 can fail, although both of these cases are unlikely with proper use. In general, if it is possible to upload code to the Teensy, the Teensy is still working. Watch for significant heat generation as a precursor to Teensy failure. In general, they should not be noticably above room temperature. MCP2561 failure is generally marked on the Teensy side by the previously mentioned 8 second delays. In addition, the PDP, PCM, and all Talons should begin flashing red.
