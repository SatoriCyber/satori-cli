pipeline {
  options {
    timeout(time: 15, unit: 'MINUTES')
    buildDiscarder(logRotator(numToKeepStr:'10'))
    disableConcurrentBuilds()
  }

  agent { label 'python-slave' } /* availbele slaves:
  the slaves are in the devops repo:
  https://github.com/SatoriCyber/devops/tree/master/chocolate-factory/jenkins_addons/slaves
  please use the relevant slave, example:
  
  agent { label 'backend-slave' }
  */

  stages {
    stage('Test') {
      steps {
        sh "./test.sh"
      }
    }

    stage('Build') {
      steps {
        sh "./build.sh"
      }
    }


  }
}
