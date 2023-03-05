import { h, render } from 'https://unpkg.com/preact?module';
import htm from 'https://unpkg.com/htm?module';

// Initialize htm with Preact
const html = htm.bind(h);

const App = ({ cpus }) => {
  return html`
    <div>
      <h1>CPUs</h1>
      <ul>
        ${Object.values(cpus).map(
          (cpu, index) =>
            html`<li>CPU ${index}: ${(cpu || 0.0).toFixed(2)}% usage</li>`,
        )}
      </ul>
    </div>
  `;
};

setInterval(async () => {
  const response = await fetch('/api/cpus');
  if (response.status !== 200) {
    throw new Error(`Error fetching CPUs: ${response.status}`);
  }

  const json = await response.json();
  render(html`<${App} cpus=${json} />`, document.body.querySelector('#app'));
}, 1000);
