/**
 * MoodleService provides utilities for working with Moodle versions and configurations
 */
export class MoodleService {
  /**
   * Returns a list of available Moodle branches
   */
  static getAvailableBranches() {
    return [
      // Current stable branches
      'MOODLE_500_STABLE',
      'MOODLE_405_STABLE',
      'MOODLE_404_STABLE',
      'MOODLE_403_STABLE',
      'MOODLE_402_STABLE',
      'MOODLE_401_STABLE',
      // Older stable branches now included
      'MOODLE_400_STABLE',
      'MOODLE_311_STABLE',
      'MOODLE_310_STABLE',
      'MOODLE_39_STABLE',
      // Primary development branch
      'master'
    ];
  }

  /**
   * Validates a site name
   */
  static validateSiteName(siteName: string) {
    if (!siteName || siteName.trim() === '') {
      return {
        isValid: false,
        message: 'Site name is required'
      };
    }

    // Regular expression to validate site name (alphanumeric, underscore, hyphen)
    const validNameRegex = /^[a-zA-Z0-9_-]+$/;
    if (!validNameRegex.test(siteName)) {
      return {
        isValid: false,
        message: 'Site name can only contain letters, numbers, underscores, and hyphens'
      };
    }

    return {
      isValid: true,
      message: ''
    };
  }

  /**
   * Updates PHP version based on Moodle branch
   */
  static updatePhpVersionForBranch(branch: string) {
    // Define compatibility rules for different Moodle branches
    let recommendedVersion = '8.2';
    let isCompatible = true;
    let currentVersion = recommendedVersion;

    // Older versions (3.9, 3.10) need PHP 7.x
    if (branch.includes('39') || branch.includes('310')) {
      recommendedVersion = '7.4';
      isCompatible = true; // Will be properly checked in isPhpVersionCompatible method
    } 
    // 3.11 and 4.0 can use PHP 8.0
    else if (branch.includes('311') || branch.includes('400')) {
      recommendedVersion = '8.0';
      isCompatible = true;
    }
    // 4.1 can use PHP 8.1
    else if (branch.includes('401')) {
      recommendedVersion = '8.1';
      isCompatible = true;
    }
    // 4.2 and 4.3 can use PHP 8.1-8.2
    else if (branch.includes('402') || branch.includes('403')) {
      recommendedVersion = '8.1';
      isCompatible = true;
    }
    // 4.4 and 4.5 can use PHP 8.1-8.3
    else if (branch.includes('404') || branch.includes('405')) {
      recommendedVersion = '8.2';
      isCompatible = true;
    }
    // Moodle 5.0 requires PHP 8.2 to 8.4
    else if (branch.includes('500')) {
      recommendedVersion = '8.2';
      isCompatible = true;
    }
    // Master branch (development) uses latest PHP
    else if (branch === 'master') {
      recommendedVersion = '8.4';
      isCompatible = true;
    }
    // Default for other versions
    else {
      recommendedVersion = '8.2';
      isCompatible = true;
    }

    return {
      recommendedVersion,
      isCompatible,
      currentVersion: recommendedVersion
    };
  }

  /**
   * Checks if a specific PHP version is compatible with a Moodle branch
   */
  static isPhpVersionCompatible(phpVersion: string, moodleBranch: string): boolean {
    const php = parseFloat(phpVersion);
    
    // Moodle 3.9: PHP 7.2 - 7.4
    if (moodleBranch.includes('39')) {
      return php >= 7.2 && php <= 7.4;
    }
    
    // Moodle 3.10: PHP 7.2 - 7.4
    if (moodleBranch.includes('310')) {
      return php >= 7.2 && php <= 7.4;
    }
    
    // Moodle 3.11: PHP 7.4 - 8.0
    if (moodleBranch.includes('311')) {
      return php >= 7.4 && php <= 8.0;
    }
    
    // Moodle 4.0: PHP 7.4 - 8.1
    if (moodleBranch.includes('400')) {
      return php >= 7.4 && php <= 8.1;
    }
    
    // Moodle 4.1: PHP 8.0 - 8.2
    if (moodleBranch.includes('401')) {
      return php >= 8.0 && php <= 8.2;
    }
    
    // Moodle 4.2, 4.3: PHP 8.0 - 8.2
    if (moodleBranch.includes('402') || moodleBranch.includes('403')) {
      return php >= 8.0 && php <= 8.2;
    }
    
    // Moodle 4.4, 4.5: PHP 8.1 - 8.3
    if (moodleBranch.includes('404') || moodleBranch.includes('405')) {
      return php >= 8.1 && php <= 8.3;
    }
    
    // Moodle 5.0: PHP 8.2 - 8.4
    if (moodleBranch.includes('500')) {
      return php >= 8.2 && php <= 8.4;
    }
    
    // Master branch: PHP 8.1+
    if (moodleBranch === 'master') {
      return php >= 8.1;
    }
    
    // Default: assume compatible for unknown versions
    return true;
  }

  /**
   * Returns default site configuration
   */
  static getDefaultSiteConfig() {
    return {
      debug: true,
      devLibs: true,
      disableCaching: true,
      disableThemeCaching: true
    };
  }

  /**
   * Returns default admin account settings
   */
  static getDefaultAdminAccount() {
    return {
      username: 'admin',
      password: 'Admin123!',
      email: 'admin@example.com',
      fullname: 'Admin User'
    };
  }

  /**
   * Returns default Docker configuration
   */
  static getDefaultDockerConfig() {
    return {
      network: 'moodlenet',
      containerPrefix: 'moodle-site',
      composePath: '/usr/local/bin/docker-compose'
    };
  }

  /**
   * Returns default PHP configuration
   */
  static getDefaultPhpConfig() {
    return {
      memoryLimit: '512M',
      additionalModules: '',
      xdebug: false
    };
  }

  /**
   * Returns available PHP versions
   */
  static getAvailablePhpVersions() {
    return ['8.4', '8.3', '8.2', '8.1', '8.0', '7.4', '7.3', '7.2', '7.1'];
  }

  /**
   * Returns available database types
   */
  static getAvailableDbTypes() {
    return ['MySQL', 'MariaDB', 'PostgreSQL', 'MSSQL'];
  }
}
