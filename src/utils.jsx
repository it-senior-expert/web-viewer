export function getUrlById(data, targetId) {
    const item = data.find(obj => obj.id === targetId); // Find the object with the matching id
    return item ? item.url : null; // Return the url if found, otherwise null
}