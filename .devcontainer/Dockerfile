FROM  archlinux:latest

# Install dependencies
RUN	 pacman -Syyu --noconfirm sudo openssh bash-completion nano git

# Generate host keys
RUN  /usr/bin/ssh-keygen -A

# Add defualt user
RUN  useradd vscode
RUN  mkdir -p /home/vscode/.ssh
RUN  chmod -R 700 /home/vscode/.ssh/
RUN  echo "YOUR_SSH_PUBLIC_KEY" > home/vscode/.ssh/authorized_keys
RUN  chmod 600 /home/vscode/.ssh/authorized_keys
RUN  chown -R vscode:vscode /home/vscode/
RUN  usermod -aG wheel vscode
RUN	 echo 'vscode:1234' | chpasswd

# Add password to root user
RUN	 echo 'root:1234' | chpasswd

# Config sshd
RUN  sed -i -e 's/^#PasswordAuthentication yes/PasswordAuthentication no/g' /etc/ssh/sshd_config
RUN  sed -i -e 's/^#PubkeyAuthentication yes/PubkeyAuthentication yes/g' /etc/ssh/sshd_config
RUN  sed -i -e 's/^UsePAM yes/UsePAM no/g' /etc/ssh/sshd_config

# Run openssh daemon
CMD  ["/usr/sbin/sshd", "-D"]

# Set default user
USER  vscode
