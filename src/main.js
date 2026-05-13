document.addEventListener('DOMContentLoaded', () => {
  const tabs = document.querySelectorAll('.tab');
  const panes = document.querySelectorAll('.tab-pane');

  tabs.forEach(tab => {
    tab.addEventListener('click', () => {
      const target = tab.dataset.tab;

      tabs.forEach(t => t.classList.remove('active'));
      panes.forEach(p => p.classList.remove('active'));

      tab.classList.add('active');
      document.getElementById(target).classList.add('active');

      // Lazy-load cowork tab on first click
      const coworkFrame = document.getElementById('cowork');
      if (target === 'cowork' && coworkFrame.src === 'about:blank') {
        coworkFrame.src = 'https://claude.ai';
      }
    });
  });
});
