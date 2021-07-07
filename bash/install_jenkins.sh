#!/bin/bash

# USAGE: If you want to start jenkins after install add start as parameter
# if you want to let port 8080 throw firefall add firewall as parameter
# you can both seperatly or in any order
 		 		

# BIG FAT WARNING!!!!   NOT TESTED ON OPENSUSE

echo "=================== STARTED JENKINS WIZARD ============================="
echo "   "
echo "   "

ARG=$1
ARG1=$2

RHET="rhel"
DEBIAN="debian"
UBUNTU="ubuntu"
CENTOS="centos"
SUSE="opensuse"

if (( EUID != 0 )); then
   echo "You must be root to do this." 1>&2
   exit 100
fi

if java -version | grep -q "java version" ; then
  which java
else
  echo "Java NOT installed!"
  echo "You should install Java"
fi

cd /
. /etc/os-release

DISTRO="echo $ID"

if [[ "echo $RHET" == "$DISTRO" ]] || [[ "echo $CENTOS" == "$DISTRO" ]]; then
	wget -O /etc/yum.repos.d/jenkins.repo http://pkg.jenkins-ci.org/redhat/jenkins.repo
	rpm --import https://jenkins-ci.org/redhat/jenkins-ci.org.key
	yum install -y jenkins
	echo "Done"
elif [[ "echo $DEBIAN" == "$DISTRO" ]] || [[ "echo $UBUNTU" == "$DISTRO" ]]; then
	wget -q -O - https://pkg.jenkins.io/debian/jenkins.io.key | yum add -
	sh -c 'echo deb http://pkg.jenkins.io/debian-stable binary/ > /etc/apt/sources.list.d/jenkins.list'
	apt-get update -y
	apt-get install -y jenkins
	echo "Done"

elif [[ "echo $SUSE" == "$DISTRO" ]]; then
	zypper addrepo http://pkg.jenkins-ci.org/opensuse/ jenkins
	zypper install jenkins
	echo "Done"
else
	echo " "
	echo "Unknow distribution!"
	exit 100
fi



if [[ $ARG == "firewall" ]] || [[ $ARG1 == "firewall" ]]; then
	firewall-cmd --zone=public --add-port=8080/tcp --permanent
	firewall-cmd --zone=public --add-service=http --permanent
	firewall-cmd --reload
	echo ""
else
	echo "You can enable firewall on port 80/8080 by following commands"
	echo ""
	echo "firewall-cmd --zone=public --add-port=8080/tcp --permanent"
	echo ""
	echo "firewall-cmd --zone=public --add-service=http --permanent"
	echo ""
	echo "firewall-cmd --reload"
	echo ""
	echo "FIREWALL ENABLED"
fi


if [[ $ARG == "start" ]] || [[ $ARG1 == "start" ]]; then
	systemctl start jenkins
	systemctl enable jenkins
	echo "Jenkins started on default port 8080"
	echo "   "
	echo "    "
	echo "To change defualt port of Jenkins go to Jenkins instalation folder and change port in Jenkins.xml and restart Jenkins"
else
	echo "You can start jenkins with = systemctl start jenkins"
	echo ""
	echo "You can enable jenkins with = systemctl enable jenkins"
	echo ""
	echo "To change defualt port of Jenkins go to Jenkins instalation folder and change port in Jenkins.xml"
fi


echo "You can access Jenkins on http://<your server IP(or localhost)>:8080"
echo ""
echo "=========================  JENKINS WIZARD FINISHED ===================================="
