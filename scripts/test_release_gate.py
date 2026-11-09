import unittest

from release_gate import release_allowed


class ReleaseGateTests(unittest.TestCase):
    def setUp(self):
        self.run = {
            "conclusion": "success",
            "head_branch": "main",
            "event": "push",
            "head_repository": {"full_name": "BRIDGE-ALLAH/MOONSHIPS"},
            "head_sha": "a" * 40,
        }

    def test_allows_successful_push_from_main_repository(self):
        self.assertTrue(release_allowed(self.run, "BRIDGE-ALLAH/MOONSHIPS"))

    def test_rejects_failed_ci(self):
        self.run["conclusion"] = "failure"
        self.assertFalse(release_allowed(self.run, "BRIDGE-ALLAH/MOONSHIPS"))

    def test_rejects_missing_or_cancelled_ci(self):
        for conclusion in (None, "cancelled", "skipped"):
            with self.subTest(conclusion=conclusion):
                self.run["conclusion"] = conclusion
                self.assertFalse(release_allowed(self.run, "BRIDGE-ALLAH/MOONSHIPS"))

    def test_rejects_non_main_branch(self):
        self.run["head_branch"] = "feature"
        self.assertFalse(release_allowed(self.run, "BRIDGE-ALLAH/MOONSHIPS"))

    def test_rejects_pull_request_event(self):
        self.run["event"] = "pull_request"
        self.assertFalse(release_allowed(self.run, "BRIDGE-ALLAH/MOONSHIPS"))

    def test_rejects_fork_repository(self):
        self.run["head_repository"]["full_name"] = "contributor/MOONSHIPS"
        self.assertFalse(release_allowed(self.run, "BRIDGE-ALLAH/MOONSHIPS"))

    def test_rejects_missing_or_malformed_sha(self):
        for sha in (None, "", "not-a-commit"):
            with self.subTest(sha=sha):
                self.run["head_sha"] = sha
                self.assertFalse(release_allowed(self.run, "BRIDGE-ALLAH/MOONSHIPS"))


if __name__ == "__main__":
    unittest.main()
