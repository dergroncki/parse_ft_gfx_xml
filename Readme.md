# parse_ft_gfx_xml

**Retrieve tags from Rockwell FactoryTalk gfx-xml file(s)**

If you had ever the task to collect or better retrieve the tags used in a FactoryTalk View Display because the tags are scattered all over the PLC program this is the right tool for you.

**How it works**

Export the displays in question to gfx-xml: Right click the Display node inside your FactoryTalk project and follow the instruction given by FactoryTalk to export one or more displays to any folder you like.

Open the command prompt and run the programm with the path to the folder which contains the xml-files:

```
Retrieve the tags of one or more FactoryTalk View Display(s) by running parse_gfx_xml from the command line with the path to the xml-files as argument (in this case "c:\temp"):

> parse_gfx_xml c:\temp
```

You can run the tool with cargo as well:

```
Clone the project and build it with cargo. Afterwards run the project with cargo:

> cargo run c:\temp
```

All xml-files will be processed one by one and for each processed xml-file a new file which only contains the retrieved tags is generated and saved in the same folder. The files which will contain the tags have the file extension ".txt".

**Dependency**

quick-xml: [Rust high performance xml reader and writer](https://github.com/tafia/quick-xml.git)

clap: [A full featured, fast Command Line Argument Parser for Rust](https://clap.rs)


