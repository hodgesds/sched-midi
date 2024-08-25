# Sched_ext Schedulers and Tools (with MIDI support!)

# WHAT IS THIS?!
This is a fork of the [sched_ext](https://github.com/sched-ext/scx) scheduler
repo with experimental support for MIDI. The idea is to be able to interact
with schedulers via MIDI to observer and adjust the runtime properties of a
scheduler. To interface with MIDI the schedulers use the [midir](https://github.com/Boddlnagg/midir/tree/master) crate.


## Device Support
I don't have a lot of MIDI devices, so I picked up a [APC Key 25](https://www.akaipro.com/apc-key-25-mkii) 
for ~$100 USD. I chose this device because the MIDI interface was rather well
[documentmented](https://cdn.inmusicbrands.com/akai/attachments/APC%20Key%2025%20mk2%20-%20Communication%20Protocol%20-%20v1.1.pdf)
and it had decent reviews. I'm by no means a DJ or music maker so everything is
just for fun. The device has a nice RBG LED pad and 360 rotary encoders. I plan
on adding support for tuning scheduling domains via the RGB matrix and rotary
encoders.


## Running `scx_layered`

```
scx_layered --midi-port 'APC Key 25 mk2:APC Key 25 mk2 APC Key 25 mk2 C 32:1' --stats 1 f:/home/daniel/layered.json
```
sample `scx_layered` config:
```
[{
  "name":"chrome",
  "comment":"chrome",
  "matches":[
     [
	     {"CommPrefix": "chrome"}
     ],
     [
	     {"PcommPrefix": "chrome"}
     ]
  ],
  "kind": {
    "Confined":{
      "exclusive": true,
      "util_range": [0.04, 0.50]
     }
  }
},{
  "name":"ppid 1",
  "comment":"ppid 1",
  "matches":[
     [{"PPIDEquals":1}]
  ],
  "kind": {
    "Confined":{
      "exclusive": true,
      "util_range": [0.01, 0.20]
     }
  }
},{
  "name":"user",
  "comment":"user",
  "matches":[
     [{"UIDEquals":1000}]
  ],
  "kind": {
    "Confined":{
      "exclusive": true,
      "util_range": [0.01, 0.20]
     }
  }
},{
  "name":"stress-ng",
  "comment":"stress-ng",
  "matches":[
     [{"CommPrefix":"stress-ng"}],
     [{"PcommPrefix":"stress-ng"}]
  ],
  "kind": {
    "Confined":{
      "exclusive": true,
      "util_range": [0.01, 0.20]
     }
  }
},
{
  "name":"the rest",
  "comment":"the rest",
  "matches":[[]],
  "kind":{
    "Grouped": {
      "util_range": [0.05, 0.60]
    }
  }
}]
```

## Other fun ideas
In the past I experimented with [generating audio via bpf](https://github.com/hodgesds/bpftune). 
