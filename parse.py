import os , re
from pathlib import Path
from collections import Counter

env_path = '/home/bonyechi/projects/dev/history/install_test/chbv_merge'

r_installs = []

def get_packages():
    packages = []
    for filename in Path(env_path).glob('**/*.sh'):
        if filename.is_file():
           data = filename.read_text()
           parsed_data = data.replace('\n','').replace('\\','').replace('\\\\','').replace("\'","")
           stripped_data = re.sub(' +', ' ', parsed_data)
           split_by_run = stripped_data.split("RUN")
           for item in split_by_run:
               stripped_item = re.sub(' +', ' ', item).strip()
               if stripped_item != '':
                   packages.append(stripped_item)
    return packages

def console(msg,data):
    print(f"{msg} : {data}")

def process_r_packages():
    idxs = []
    all_packages = get_packages()
    console("Initial All Packages",len(all_packages))
    r_packages = []
    pip_packages = []
    apt_packages = []
    caught = []
    for idx,package in enumerate( all_packages ):
        result = re.search(r'R --no.*?save -e ".*?\(.*?\)"', package)
        if result is not None:
            idxs.append(idx)
            matched_r = result.group(0)
            r_packages.append(matched_r)
        elif "R" in package and "-e" in package:
            secondary_result = re.search(r'R -e "\s?.*?\(.*?\)"', package)
            if secondary_result is not None:
                idxs.append(idx)
                second_matched_r = secondary_result.group(0)
                r_packages.append(second_matched_r)
            else:
                caught.append(package)
        else:
            continue
    all_packages = [pkg for ids, pkg in enumerate(all_packages) if ids not in idxs]

    console("All packages (after R packages)",len(all_packages))

    pip_idxs = []
    for idx, package in enumerate(all_packages):
        result = re.search(r'pip install .*', package)
        if result is not None:
            pip_idxs.append(idx)
            matched_pip = result.group(0)
            pip_packages.append(matched_pip)
        else:
            continue

    all_packages = [pkg for ids, pkg in enumerate(all_packages) if ids not in pip_idxs]

    console("R packages: ",len(r_packages))
    console("Pip packages: ",len(pip_packages))
    console("Missed R Packages: ",len(caught))
    console("Finished all packages (after R and pip): ",len(all_packages))
    # print(all_packages)
    # print(caught)
    apt_idxs = []
    for idx, package in enumerate(all_packages):
        result = re.search(r'.*apt.*', package)
        if result is not None:
            apt_idxs.append(idx)
            matched_apt = result.group(0)
            apt_packages.append(matched_apt)
        else:
            continue

    all_packages = [pkg for ids, pkg in enumerate(all_packages) if ids not in apt_idxs]
    console("Apt packages: ",len(apt_packages))
    console("Finished all packages (after apt as well): ",len(all_packages))

    # with open('pip.pkg.log','w') as f:
    #     f.writelines("%s\n" % t for t in pip_packages)

    # with open('r.pkg.log','w') as f:
    #     f.writelines("%s\n" % t for t in r_packages)

    # with open('missed_r.pkg.log','w') as f:
    #     f.writelines("%s\n" % t for t in caught)

    # with open('apt.pkg.log','w') as f:
    #     f.writelines("%s\n" % t for t in apt_packages)

    # with open('system.pkg.log','w') as f:
    #     f.writelines("%s\n" % t for t in all_packages)

    # print(Counter(re.findall(r"[a-zA-Z./:]+","|".join(r_packages))))
    return pip_packages, r_packages, caught, apt_packages,all_packages

# print(r_installs)
# process_r_packages()

def get_pip_pkgs():
    pip, r , caught, apt, remain = process_r_packages()
    pip_pkgs = []
    for pkg in pip:
        # result = re.search(r'"(.*?)(\(.*?\))")', package)
        result = re.search(r'pip install (.*)', pkg)
        if result is not None:
            each_pkg = result.group(1)
            for each in each_pkg.split():
                if not each.startswith("-"):
                    pip_pkgs.append(each)
        # print(pkg)
    return list(set(pip_pkgs))

# print( get_pip_pkgs() )

def master_packages():
    packages = []
    filename = Path(env_path) / "master.log"
    if filename.is_file():
       data = filename.read_text()
       parsed_data = data.replace('\n','').replace('\\','').replace('\\\\','').replace("\'","")
       stripped_data = re.sub(' +', ' ', parsed_data)
       split_by_run = stripped_data.split("RUN")
       for item in split_by_run:
           stripped_item = re.sub(' +', ' ', item).strip()
           if stripped_item != '':
               packages.append(stripped_item)
    return " | ".join( packages )

def get_r_pkgs():
    pip, r , caught, apt, remain = process_r_packages()
    r_pkgs = {
        "biocmanager" : [],
        "dev_install" : [],
        "remote_install" :[],
        "normal_install": []
    }
    for pkg in r:
        bioc_result = re.search(r'BiocManager::install(\(?.*?\)?.*?\)\)?)', pkg)
        install_result = re.search(r'install.packages(\(?.*?\)?.*?\)\)?)', pkg)
        dev_result = re.search(r'devtools::install_github(\(?.*?\)?.*?\)\)?)', pkg)
        remote_result = re.search(r'remotes::install_github(\(?.*?\)?.*?\)\)?)', pkg)
        if bioc_result is not None:
            each_pkg = bioc_result.group(1)
            bioc_pkg = each_pkg.replace("c(","").replace("(","").replace(")","")
            if "," in bioc_pkg:
                bioc_pkg_list = [bio.strip() for bio in bioc_pkg.split(",") if "=" not in bio]
                r_pkgs["biocmanager"].extend(bioc_pkg_list)
            else:
                r_pkgs["biocmanager"].append(bioc_pkg.strip())
        if install_result is not None:
            each_pkg = install_result.group(1)
            install_pkg = each_pkg.replace("c(","").replace("(","").replace(")","")
            if "," in install_pkg:
                install_pkg_list = [install.strip() for install in install_pkg.split(",") if "=" not in install]
                r_pkgs["normal_install"].extend(install_pkg_list)
            else:
                r_pkgs["normal_install"].append(install_pkg.strip())
        if dev_result is not None:
            each_pkg = dev_result.group(1)
            dev_pkg = each_pkg.replace("c(","").replace("(","").replace(")","")
            if "," in dev_pkg:
                dev_pkg_list = [dev.strip() for dev in dev_pkg.split(",") if "=" not in dev]
                r_pkgs["dev_install"].extend(dev_pkg_list)
            else:
                r_pkgs["dev_install"].append(dev_pkg.strip())

        if remote_result is not None:
            each_pkg = remote_result.group(1)
            remote_pkg = each_pkg.replace("c(","").replace("(","").replace(")","")
            if "," in remote_pkg:
                remote_pkg_list = [remote.strip() for remote in remote_pkg.split(",") if "=" not in remote]
                r_pkgs["remote_install"].extend(remote_pkg_list)
            else:
                r_pkgs["remote_install"].append(remote_pkg.strip())

    r_pkgs["biocmanager"] = [ item for item in set(r_pkgs["biocmanager"]) if item not in master_packages() ]
    r_pkgs["normal_install"] = [  item for item in set(r_pkgs["normal_install"]) if item not in master_packages() ]
    r_pkgs["dev_install"] = [  item for item in set(r_pkgs["dev_install"]) if item not in master_packages() ]
    r_pkgs["remote_install"] = [  item for item in set(r_pkgs["remote_install"]) if item not in master_packages() ]


    r_pkgs["biocmanager"] = [ item for item in r_pkgs["biocmanager"] if item not in r_pkgs["normal_install"] ]
    # print(r_pkgs["biocmanager"])
    for k,v in r_pkgs.items():
        print(f"{k} : {len(v)}")
    return r_pkgs

# get_r_pkgs()
# print(master_packages())

