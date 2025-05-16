import { exec } from 'child_process';
import { promisify } from 'util';

const execPromise = promisify(exec);

export const isDockerInstalled = async (): Promise<boolean> => {
    try {
        await execPromise('docker --version');
        return true;
    } catch {
        return false;
    }
};

export const isDockerRunning = async (): Promise<boolean> => {
    try {
        const { stdout } = await execPromise('docker info');
        return stdout.includes('Server Version');
    } catch {
        return false;
    }
};

export const createDockerContainer = async (name: string, image: string, ports: { webPort: number; dbPort: number }): Promise<void> => {
    const { webPort, dbPort } = ports;
    const portMappings = [];
    if (webPort) {
        portMappings.push(`-p ${webPort}:80`);
    }
    if (dbPort) {
        portMappings.push(`-p ${dbPort}:3306`);
    }
    const command = `docker run --name ${name} ${portMappings.join(' ')} -d ${image}`;
    await execPromise(command);
};

export const stopDockerContainer = async (name: string): Promise<void> => {
    await execPromise(`docker stop ${name}`);
};

export const removeDockerContainer = async (name: string): Promise<void> => {
    await execPromise(`docker rm ${name}`);
};