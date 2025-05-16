<template>
  <div class="settings-view">
    <h1>Settings</h1>
    <form @submit.prevent="saveSettings">
      <div class="form-group">
        <label for="siteFolder">Site Folder</label>
        <input type="text" id="siteFolder" v-model="siteFolder" required />
      </div>
      <div class="form-group">
        <label for="phpVersion">PHP Version</label>
        <select id="phpVersion" v-model="phpVersion">
          <option v-for="version in phpVersions" :key="version" :value="version">{{ version }}</option>
        </select>
      </div>
      <div class="form-group">
        <label for="dbType">Database Type</label>
        <select id="dbType" v-model="dbType">
          <option v-for="type in dbTypes" :key="type" :value="type">{{ type }}</option>
        </select>
      </div>
      <div class="form-group">
        <label for="moodleBranch">Moodle Branch Version</label>
        <select id="moodleBranch" v-model="moodleBranch">
          <option v-for="branch in moodleBranches" :key="branch" :value="branch">{{ branch }}</option>
        </select>
      </div>
      <div class="form-group">
        <label for="dbPort">DB Port</label>
        <input type="number" id="dbPort" v-model="dbPort" placeholder="Auto-generated if left blank" />
      </div>
      <div class="form-group">
        <label for="webPort">Web Port</label>
        <input type="number" id="webPort" v-model="webPort" placeholder="Auto-generated if left blank" />
      </div>
      <button type="submit">Create</button>
    </form>
    <div class="logs">
      <h2>Logs</h2>
      <pre>{{ logs }}</pre>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      siteFolder: '',
      phpVersion: '',
      dbType: '',
      moodleBranch: '',
      dbPort: '',
      webPort: '',
      phpVersions: ['7.4', '8.0', '8.1'],
      dbTypes: ['MySQL', 'PostgreSQL'],
      moodleBranches: ['MOODLE_39_STABLE', 'MOODLE_40_STABLE'],
      logs: ''
    };
  },
  methods: {
    saveSettings() {
      // Logic to save settings and create the site
      this.logs += `Creating site in ${this.siteFolder} with PHP ${this.phpVersion}, DB ${this.dbType}, Moodle ${this.moodleBranch}\n`;
      // Additional logic for port handling and site creation
    }
  }
};
</script>

<style scoped>
.settings-view {
  padding: 20px;
}
.form-group {
  margin-bottom: 15px;
}
</style>