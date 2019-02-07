pipeline {
    agent {
        label 'rhel6'
    }
    environment {
        PATH = "$PATH:$HOME/.cargo/bin"
    }
    stages {
        stage('rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are
                // required.
                sh "cargo fmt --all -- --check"
            }
        }
        stage('build') {
            steps {
                sh "cargo build"
            }
        }
        stage('test') {
            steps {
                sh "cargo test"
            }
        }
        stage('clippy') {
            steps {
                sh "cargo clippy --all"
            }
        }
        stage('rpm') {
            steps {
                sh "cargo rpm init"
                sh "cargo rpm build"
            }
        }
    }
}
