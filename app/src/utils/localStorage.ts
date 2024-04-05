import { useLocalStorage } from "usehooks-ts";

const STORAGE_GH_CODE_KEY = 'gh_code';
const STORAGE_FP_SESSION_KEY = 'fp_session';

export function useLocalSession() {
  function clear<T>(key: string, handleSave: (value: T | null) => void) {
    handleSave(null);
    localStorage.removeItem(key);
  }

  const [githubCode, saveGithubCode] = useLocalStorage<string | null>(STORAGE_GH_CODE_KEY, null);
  const clearGithubCode = () => clear(STORAGE_GH_CODE_KEY, saveGithubCode);

  const [sessionId, saveSessionId] = useLocalStorage<string | null>(STORAGE_FP_SESSION_KEY, null);
  const clearSessionId = () => clear(STORAGE_FP_SESSION_KEY, saveSessionId);

  return {githubCode, saveGithubCode, clearGithubCode, sessionId, saveSessionId, clearSessionId };
}