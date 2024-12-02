document.addEventListener('DOMContentLoaded', function() {
  function toggleInputs() {
      const bareCheckbox = document.getElementById('bare');
      const separateGitDirInput = document.getElementById('separate-git-dir');

      if (bareCheckbox.checked) {
          separateGitDirInput.disabled = true;
      } else {
          separateGitDirInput.disabled = false;
      }

      if (separateGitDirInput.value.trim() !== '') {
          bareCheckbox.disabled = true;
      } else {
          bareCheckbox.disabled = false;
      }
  }

  function generateCommand() {
      const form = document.getElementById('git-init-form');
      let command = 'git init';

      if (form.quiet.checked) {
          command += ' --quiet';
      }

      if (form.bare.checked) {
          command += ' --bare';
      }

      const template = form.template.value.trim();
      if (template) {
          command += ` --template=${template}`;
      }

      const separateGitDir = form['separate-git-dir'].value.trim();
      if (separateGitDir) {
          command += ` --separate-git-dir=${separateGitDir}`;
      }

      const objectFormat = form['object-format'].value;
      command += ` --object-format=${objectFormat}`;

      const refFormat = form['ref-format'].value;
      command += ` --ref-format=${refFormat}`;

      const branchName = form['branch-name'].value.trim();
      if (branchName) {
          command += ` --initial-branch=${branchName}`;
      }

      const shared = form.shared.value;
      command += ` --shared=${shared}`;

      const directory = form.directory.value.trim();
      if (directory) {
          command += ` ${directory}`;
      }

      document.getElementById('command-output').textContent = command;
  }

  const bareCheckbox = document.getElementById('bare');
  const separateGitDirInput = document.getElementById('separate-git-dir');

  bareCheckbox.addEventListener('change', toggleInputs);
  separateGitDirInput.addEventListener('input', toggleInputs);

  document.getElementById('git-init-form').addEventListener('submit', function(event) {
      event.preventDefault();
      generateCommand();
  });
});
