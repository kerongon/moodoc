import axios from 'axios';

const MOODLE_VERSIONS_API = 'https://moodle.org/api/v1/versions'; // Example API endpoint for fetching Moodle versions

export async function fetchMoodleVersions() {
    try {
        const response = await axios.get(MOODLE_VERSIONS_API);
        return response.data; // Assuming the API returns an array of versions
    } catch (error) {
        console.error('Error fetching Moodle versions:', error);
        throw error; // Rethrow the error for handling in the calling function
    }
}

export function validateMoodleVersion(version) {
    const validVersions = ['3.9', '3.10', '3.11', '4.0']; // Example valid versions
    return validVersions.includes(version);
}