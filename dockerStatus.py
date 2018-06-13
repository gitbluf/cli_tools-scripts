

import requests, json, pprint, sys
import urllib3

urllib3.disable_warnings()

if len (sys.argv) != 2 :
    print "Usage: python dockerStatus.py <environment_name> "
    sys.exit (1)

def getAuthToken(hostName):
    payload = {"username": "user", "password": "*******".decode('base64')}
    authTokenResp = requests.post(hostName + '/auth/login',data=json.dumps(payload),verify=False)
    auth_token = authTokenResp.json()['auth_token']
    return auth_token

environment_name = sys.argv[1]

idev_host = 'https://sea-ucp.davita.corp'
qa_host = 'https://qa-sea-ucp.davita.corp'
stage_host = 'https://den-ucp.davita.corp'
prod_host = 'https://prod-den-ucp.davita.corp'
host = ''

if(environment_name == 'idev' or environment_name == 'dev'):
    host = idev_host
elif(environment_name == 'qa'):
    host = qa_host
elif(environment_name == 'stage'):
    host = stage_host
elif(environment_name == 'prod'):
    host = prod_host
else:
    print 'Usage: Wrong environment name provided. Provide one of the following: dev, idev, qa, stage, prod'
    sys.exit (1)

bearer = 'Bearer ' + getAuthToken(host)
headers = {'Authorization': bearer, 'accept': 'application/json'}

nodesResp = requests.get(host + '/nodes',headers=headers,verify=False)
nodes = nodesResp.json()
print 'Total number of nodes = ', len(nodes)
num_manager_nodes = 0
num_manager_nodes_running = 0
num_worker_nodes = 0
num_worker_nodes_running = 0
for node in nodes:
    if(node['Spec']['Role'] == 'manager'):
        num_manager_nodes += 1
        if(node['Spec']['Availability'] == 'active'):
            num_manager_nodes_running += 1
    if(node['Spec']['Role'] == 'worker'):
        num_worker_nodes += 1
        if(node['Spec']['Availability'] == 'active'):
            num_worker_nodes_running += 1

print str(num_manager_nodes_running) + '/' + str(num_manager_nodes) + ' manager nodes running'
print str(num_worker_nodes_running) + '/' + str(num_worker_nodes) + ' worker nodes running'
print ''
print ''

#kontenjeri
containersResp = requests.get(host + '/containers/json',headers=headers,verify=False)
containers = containersResp.json()

#servisi
servicesResp = requests.get(host + '/services',headers=headers,verify=False)
services = servicesResp.json()


num_services = 0
servicesDict = {}
for service in services:
    serviceName = service['Spec']['Name']
    serviceEnvironmentLabel = ''
    try:
        serviceEnvironmentLabel = service['Spec']['Labels']['com.docker.ucp.access.label']
    except:
        serviceEnvironmentLabel = 'NA'
    if(serviceEnvironmentLabel == environment_name):
        num_services += 1
        namespace = ''
        try:
            namespace = service['Spec']['Labels']['com.docker.stack.namespace']
        except:
            namespace = 'None'
        num_running_containers = 0
        for container in containers:
            containerName = container['Names'][0]
            serviceNameinContainer = ''
            try:
                serviceNameinContainer = container['Labels']['com.docker.swarm.service.name']
            except:
                serviceNameinContainer = 'NA'
            if(containerName.find(environment_name) != -1):
                if(serviceNameinContainer == serviceName):
                    num_running_containers += 1

        servicesDict.setdefault(namespace,[]).append({"serviceName":serviceName, "containers":num_running_containers})

for key, svcs in servicesDict.iteritems():
    totalServicesFound = len(svcs)
    runningSvcs = 0
    stoppedSvcs = []
    totalContainers = 0
    for svc in svcs:
        if (svc['containers'] > 0):
            runningSvcs += 1
            totalContainers += svc['containers']
        else:
            stoppedSvcs.append(svc)
    print key + ' : ' + str(runningSvcs) + '/' + str(totalServicesFound) + ' services running in '+ str(totalContainers) + ' containers'
    for stoppedSvc in stoppedSvcs:
        print stoppedSvc['serviceName'] + ' is running on no container'
    print ""