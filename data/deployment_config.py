import xml.dom.minidom as minidom

# Ask user for input
dirs = input("マッピング対象ディレクトリを入力してください(複数入力する場合はカンマ、スペース等で区切る)\n").replace(' ', ',').split(',')
dirs = [dir.strip() for dir in dirs]

npm_dirs = []
for dir in dirs:
    is_npm = input(f"{dir}ディレクトリはnpmパッケージですか？[Y/n]").lower()
    if is_npm == 'y' or not is_npm:
        npm_dirs.append(dir)

# Create an xml file based on the above info
doc = minidom.Document()

project = doc.createElement('project')
project.setAttribute('version', '4')
doc.appendChild(project)

component = doc.createElement('component')
component.setAttribute('name', 'PublishConfigData')
component.setAttribute('autoUpload', 'Always')
component.setAttribute('serverName', 'remote')
component.setAttribute('remoteFilesAllowedToDisappearOnAutoupload', 'false')
component.setAttribute('confirmBeforeUploading', 'false')
project.appendChild(component)

serverData = doc.createElement('serverData')
component.appendChild(serverData)

paths = doc.createElement('paths')
paths.setAttribute('name', 'remote')
serverData.appendChild(paths)

serverdata = doc.createElement('serverdata')
paths.appendChild(serverdata)

mappings = doc.createElement('mappings')
serverdata.appendChild(mappings)

excludedPaths = doc.createElement('excludedPaths')
serverdata.appendChild(excludedPaths)

for dir in dirs:
    mapping = doc.createElement('mapping')
    mapping.setAttribute('deploy', f"/home/vagrant/{dir}")
    mapping.setAttribute('local', f"$PROJECT_DIR$/{dir}")
    mapping.setAttribute('web', f"/{dir}")
    mappings.appendChild(mapping)

    if dir in npm_dirs:
        excludedPath1 = doc.createElement('excludedPath')
        excludedPath1.setAttribute('path', f"/home/vagrant/{dir}/node_modules")
        excludedPaths.appendChild(excludedPath1)

        excludedPath2 = doc.createElement('excludedPath')
        excludedPath2.setAttribute('local', 'true')
        excludedPath2.setAttribute('path', f"$PROJECT_DIR$/{dir}/node_modules")
        excludedPaths.appendChild(excludedPath2)

option = doc.createElement('option')
option.setAttribute('name', 'myAutoUpload')
option.setAttribute('value', 'ALWAYS')
component.appendChild(option)

# Create an xml tree and save it to a file
xml_str = doc.toprettyxml(indent="  ")
with open("../.idea/deployment.xml", "w") as f:
    f.write(xml_str)

print("config file is generated.")
