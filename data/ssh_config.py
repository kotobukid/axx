import os
import subprocess
import xml.etree.ElementTree as ET
import uuid

def generate_ssh_config(uuid_str):
    result = subprocess.run(['vagrant', 'ssh-config'], stdout=subprocess.PIPE)
    lines = result.stdout.decode().split('\n')
    config = {line.split()[0]: line.split()[1] for line in lines if line}

    root = ET.Element("project")
    root.set("version", "4")

    component = ET.SubElement(root, "component")
    component.set("name", "SshConfigs")

    configs = ET.SubElement(component, "configs")

    sshConfig = ET.SubElement(configs, "sshConfig")
    sshConfig.set("host", config['HostName'])
    sshConfig.set("id", uuid_str)
    sshConfig.set("keyPath", "$PROJECT_DIR$/.vagrant/machines/default/virtualbox/private_key")
    sshConfig.set("port", config['Port'])
    sshConfig.set("nameFormat", "DESCRIPTIVE")
    sshConfig.set("username", config['User'])
    sshConfig.set("useOpenSSHConfig", "true")

    tree = ET.ElementTree(root)
    tree.write("../.idea/sshConfigs.xml")

def generate_web_servers(uuid_str):
    root = ET.Element("project")
    root.set("version", "4")

    component = ET.SubElement(root, "component")
    component.set("name", "WebServers")

    option = ET.SubElement(component, "option")
    option.set("name", "servers")

    webServer = ET.SubElement(option, "webServer")
    webServer.set("id", str(uuid.uuid4()))
    webServer.set("name", "remote")

    fileTransfer = ET.SubElement(webServer, "fileTransfer")
    fileTransfer.set("accessType", "SFTP")
    fileTransfer.set("host", "127.0.0.1")
    fileTransfer.set("port", "2222")
    fileTransfer.set("sshConfigId", uuid_str)
    fileTransfer.set("sshConfig", "vagrant@127.0.0.1:2222 key")
    fileTransfer.set("keyPair", "true")

    advancedOptions1 = ET.SubElement(fileTransfer, "advancedOptions")

    advancedOptions = ET.SubElement(advancedOptions1, "advancedOptions")
    advancedOptions.set("dataProtectionLevel", "Private")
    advancedOptions.set("keepAliveTimeout", "0")
    advancedOptions.set("passiveMode", "true")
    advancedOptions.set("shareSSLContext", "true")

    tree = ET.ElementTree(root)
    tree.write("../.idea/webServers.xml")

uuid_str = str(uuid.uuid4())
generate_ssh_config(uuid_str)
generate_web_servers(uuid_str)
