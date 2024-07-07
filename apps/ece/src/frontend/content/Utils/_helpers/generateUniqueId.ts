let currentId = 0;

export const generateUniqueId = (): number => {
  return currentId++;
};