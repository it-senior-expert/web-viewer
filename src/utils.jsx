export function getUrlById(data, targetId) {
    const item = data.find(obj => obj.id === targetId); // Find the object with the matching id
    return item ? item.url : null; // Return the url if found, otherwise null
}
export function isValidUrl(str) {
    try {
        new URL(str);
        return true;
    } catch (_) {
        return false;
    }
};
export function saveToLocalStorage(key, value) {
    try {
      localStorage.setItem(key, JSON.stringify(value));
      console.log(`Data saved to localStorage under the key: ${key}`);
    } catch (error) {
      console.error('Error saving to localStorage:', error);
    }
  }
  
  export function loadFromLocalStorage(key) {
    try {
      const data = localStorage.getItem(key);
      return data ? JSON.parse(data) : null;
    } catch (error) {
      console.error('Error loading from localStorage:', error);
      return null;
    }
  }
  