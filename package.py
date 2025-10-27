import os
import shutil
import tomllib
import subprocess

dir_project = os.path.dirname(__file__)
cargo_path = os.path.join(dir_project, 'Cargo.toml')

with open(cargo_path, 'rb') as cargo:
    cargo = tomllib.load(cargo)
    
name_package = cargo['package']['name']
version_package = cargo['package']['version']

dir_package = os.path.join(dir_project, name_package)
print("Diretório de empacotamento será gerado em:")
print(dir_package)
print('')

if os.path.exists(dir_package):
    shutil.rmtree(dir_package)

os.mkdir(dir_package)
print('Criei o diretório de empacotamento')
print(
    f'Caminho completo: \n{dir_package} | CONDITION: {os.path.exists(dir_package)}\n'
    )

DEBIAN = os.path.join(dir_package, 'DEBIAN')
control = os.path.join(DEBIAN, 'control')
usr = os.path.join(dir_package, 'usr')
bin_ = os.path.join(usr, 'bin')


os.mkdir(DEBIAN)
with open(control, 'w', encoding='utf-8') as contr:
    contr.write(f"""
Package: {name_package}
Priority: optional
Section: misc
Maintainer: Érick Alves <onemylinux@gmail.com>
Architecture: amd64
Version: {version_package}
Description: Esse é um simples pacote para manipular cores
""".strip() + '\n')

os.mkdir(usr)
os.mkdir(bin_)

comands = ['cargo', 'build', '--release']
subprocess.run(
    comands,
    capture_output=True
)

comands = ' '.join(comands)
print(f"Compilando com:\n{comands}")

copy_binary = os.path.join(bin_, name_package)
target = os.path.join(dir_project, 'target')
release = os.path.join(target, 'release')
binary = os.path.join(release, name_package)
if os.path.exists(binary):
    print('Copiando o binário release para empacotar...\n')
    shutil.copy(binary, copy_binary)

comand = ['tree', dir_package + '/']
comand = subprocess.run(
    comand,
    capture_output=True,
    text=True
)

print("Saída da árvore:")
print(comand.stdout)

print('Empacotando com dpkg...')
file = os.path.join(dir_project, name_package + f'-{version_package}_.deb')

comand = [
    'fakeroot',
    'dpkg',
    '-b',
    dir_package,
    file
]

subprocess.run(comand)
print('')

print('Removendo diretório de empacotamento...')
print(dir_package)
shutil.rmtree(dir_package)