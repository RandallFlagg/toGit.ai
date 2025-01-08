// Sample data with timestamps (not changed as per request)
//real data sample
// const gitData = {
// "{\"nodes\":{\"1\":{\"id\":1,\"name\":\"interface\",\"commit_sha\":\"813fce01b5023758bbcb9f8e9a80e16314060a63\",\"node_type\":\"LocalBranch\",\"timestamp\":\"2024-06-27T22:40:59Z\",\"parent1\":\"8be159f79c52c0c2314198188cd31178145ed357\",\"parent2\":null,\"parents\":[],\"num_children\":0,\"children\":[],\"firstchild\":null,\"x\":50,\"y\":400},\"2\":{\"id\":2,\"name\":\"Lior_master\",\"commit_sha\":\"41d88f211e9d9d05ef0c4b412b28914afa42330e\",\"node_type\":\"LocalBranch\",\"timestamp\":\"2024-11-24T14:14:02Z\",\"parent1\":\"2a563eacb706d49eb086ae27ed4c257980fe1b01\",\"parent2\":null,\"parents\":[],\"num_children\":0,\"children\":[],\"firstchild\":null},\"3\":{\"id\":3,\"name\":\"master\",\"commit_sha\":\"3dafffb9ca148a952d8e260b87172d83913c93f8\",\"node_type\":\"LocalBranch\",\"timestamp\":\"2023-12-04T20:34:24Z\",\"parent1\":\"af3b7867aada8e4954a0b849c83d5b7287525201\",\"parent2\":null,\"parents\":[],\"num_children\":0,\"children\":[],\"firstchild\":null},\"4\":{\"id\":4,\"name\":\"MINGW_FIXES\",\"commit_sha\":\"391ffd700c044b0e7773e2c8362605273f9c52fb\",\"node_type\":\"LocalBranch\",\"timestamp\":\"2024-06-20T13:39:03Z\",\"parent1\":\"9d00b656f07c6ed47cb4aa7c51a2fcc97e28a643\",\"parent2\":null,\"parents\":[],\"num_children\":1,\"children\":[8],\"firstchild\":\"755a8879e331c88fd31d47e2d4dd8cb6b251c3f8\"},\"5\":{\"id\":5,\"name\":\"READ_ME\",\"commit_sha\":\"5545a397bd6cbc1d605fffb48209ad044cb0c4b0\",\"node_type\":\"LocalBranch\",\"timestamp\":\"2020-05-21T21:43:46Z\",\"parent1\":\"234dfe2c84372b471987a396149c5a4234cf5041\",\"parent2\":\"3621edb94cc1b71ddfee1b92a65b78476de31eb4\",\"parents\":[],\"num_children\":0,\"children\":[],\"firstchild\":null},\"6\":{\"id\":6,\"name\":\"Wayland\",\"commit_sha\":\"0aafb42865644566f156b857e32ef1440f43bf2d\",\"node_type\":\"LocalBranch\",\"timestamp\":\"2023-12-05T03:10:29Z\",\"parent1\":\"b7d57e1f065d712ad3e1f04488f8fb61b21d574e\",\"parent2\":null,\"parents\":[],\"num_children\":0,\"children\":[],\"firstchild\":null},\"7\":{\"id\":7,\"name\":\"Win32Interface\",\"commit_sha\":\"cd21507dda5e1388776b0a9b1cce1dbc7a64b3ad\",\"node_type\":\"LocalBranch\",\"timestamp\":\"2023-12-18T01:04:39Z\",\"parent1\":\"af3b7867aada8e4954a0b849c83d5b7287525201\",\"parent2\":null,\"parents\":[],\"num_children\":0,\"children\":[],\"firstchild\":null},\"8\":{\"id\":8,\"name\":\"origin/RUST\",\"commit_sha\":\"755a8879e331c88fd31d47e2d4dd8cb6b251c3f8\",\"node_type\":\"RemoteBranch\",\"timestamp\":\"2024-11-24T12:16:37Z\",\"parent1\":\"391ffd700c044b0e7773e2c8362605273f9c52fb\",\"parent2\":null,\"parents\":[4],\"num_children\":0,\"children\":[],\"firstchild\":null}},\"colors\":{\"local_branch\":\"#00FF00\",\"active_branch\":\"#FF0000\",\"remote_branch\":\"#F5F5DC\",\"remote_tag\":\"#FFFF00\",\"background\":\"#FFFFFF\",\"local_tag\":\"#FFFFE0\"}}"
// };
// Initialize the graph on page load
document.addEventListener("DOMContentLoaded", async () => {
  let gitData;
  async function fetchGitData() {
    const repo_path = "~/Ohad/Projects/OIV/OIV/"; // TODO: Adjust as needed
    const response = await window.__TAURI__.core.invoke('get_git_data', { repoPath: repo_path }); //TODO: From where should we get the repo path?
    if (false) {
      const MygitData = {
        nodes: {
          1: { id: 1, name: "interface", commit_sha: "813fce01b5023758bbcb9f8e9a80e16314060a63", node_type: "LocalBranch", timestamp: "2024-06-27T22:40:59Z", parent1: "8be159f79c52c0c2314198188cd31178145ed357", parent2: null, parents: [2], num_children: 0, children: [], firstchild: null, x: 50, y: 400 },
          2: { id: 2, name: "Lior_master", commit_sha: "41d88f211e9d9d05ef0c4b412b28914afa42330e", node_type: "LocalBranch", timestamp: "2024-11-24T14:14:02Z", parent1: "2a563eacb706d49eb086ae27ed4c257980fe1b01", parent2: null, parents: [], num_children: 0, children: [1], firstchild: null },
        },
        colors: {
          local_branch: "#00FF00", active_branch: "#FF0000", remote_branch: "#F5F5DC", remote_tag: "#FFFF00", background: "#FFFFFF", local_tag: "#FFFFE0"
        }
      };
      const response = MygitData;
    }
    // console.log("Colors: " + response.colors.remote_tag);
    // const response = await window.__TAURI__.core;
    console.log(response);
    return response;
  }

  // Function to calculate node positions based on parents and children
  function calculateNodePositions(nodes, edges) {
    const positions = {}; // Reset positions on every new calculation
    let levelSpacing = 150;  // Space between levels
    let nodeSpacing = 200;   // Space between nodes at the same level
    let yOffset = 400;       // Starting Y position for the oldest branch

    // Start positioning from the root node (oldest node)
    let nodesAtLevel = [nodes.find(node => node.parents.length === 0)]; // Oldest node at the bottom
    if (!nodesAtLevel[0]) {
      throw ("There is no parent node. Something is wrong.");
    }
    let level = 0;

    // Center the root node at the bottom
    let canvasWidth = window.innerWidth;
    let canvasHeight = window.innerHeight;

    let rootNode = nodesAtLevel[0];
    rootNode.x = canvasWidth / 2;
    rootNode.y = canvasHeight - 150; // Center at the bottom
    positions[rootNode.id] = { x: rootNode.x, y: rootNode.y };

    while (nodesAtLevel.length > 0) {
      let nextLevelNodes = [];
      nodesAtLevel.forEach((node, index) => {
        node.x = (index * nodeSpacing) + 50;
        node.y = (level * levelSpacing) + yOffset;
        positions[node.id] = { x: node.x, y: node.y };

        // Add children for the next level
        node.children.forEach(childId => {
          let childNode = nodes.find(n => n.id === childId);
          if (childNode) {
            nextLevelNodes.push(childNode);
          }
        });
      });
      nodesAtLevel = nextLevelNodes;
      level++;
    }

    return positions;
  }

  // Function to draw the graph on the canvas
  function drawGraph(nodes, edges, positions) {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    // Clear canvas before each draw
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Draw edges (links between nodes)
    edges.forEach(edge => {
      const fromNode = nodes.find(node => node.id === edge.from);
      const toNode = nodes.find(node => node.id === edge.to);

      if (fromNode && toNode) {
        const fromPos = positions[fromNode.id];
        const toPos = positions[toNode.id];

        if (fromPos && toPos) {
          ctx.beginPath();
          ctx.moveTo(fromPos.x, fromPos.y);
          ctx.lineTo(toPos.x, toPos.y);
          ctx.strokeStyle = "#999"; // Edge color
          ctx.stroke();
        } else {
          console.warn(`fromPos: ${fromPos}, toPos: ${toPos}.`);
        }
      } else {
        // Optionally log an error or handle the case where nodes are not found
        console.warn(`Edge from ${edge.from} to ${edge.to} references a non-existent node.`);
      }
    });

    // Draw nodes (rectangles with names)
    nodes.forEach(node => {
      const pos = positions[node.id];
      ctx.beginPath();
      ctx.rect(pos.x - 50, pos.y - 25, 100, 50); // Rectangular node
      ctx.fillStyle = getNodeColor(node);
      ctx.fill();
      ctx.strokeStyle = "#000";
      ctx.stroke();

      // Draw node name
      ctx.font = "12px Arial";
      ctx.fillStyle = "#000";
      ctx.fillText(node.name, pos.x - 40, pos.y); // Adjust name position
    });
  }

  // Get color for each node based on type
  function getNodeColor(node) {
    switch (node.node_type) {
      case 'LocalBranch':
        return gitData.colors.local_branch;
      case 'RemoteBranch':
        return gitData.colors.remote_branch;
      default:
        return '#888'; // Default color for unknown node types
    }
  }

  // Handle zoom functionality
  document.getElementById('zoom-in').addEventListener('click', () => {
    zoomFactor *= 1.2;
    updateGraph();
  });

  document.getElementById('zoom-out').addEventListener('click', () => {
    zoomFactor /= 1.2;
    updateGraph();
  });

  // Function to update graph based on zoom level
  function updateGraph() {
    const positions = calculateNodePositions(nodesArray, edges);

    // Apply zoom to positions
    const zoomedPositions = {};
    for (const nodeId in positions) {
      zoomedPositions[nodeId] = {
        x: positions[nodeId].x * zoomFactor,
        y: positions[nodeId].y * zoomFactor
      };
    }

    // Set canvas size to fit zoomed content
    const canvas = document.getElementById('canvas');
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    drawGraph(nodesArray, edges, zoomedPositions);
  }

  // Handle right-click context menu
  canvas.addEventListener('contextmenu', function (event) {
    event.preventDefault();

    const clickedNodeId = getNodeAtPosition(event.clientX, event.clientY);
    if (clickedNodeId !== null) {
      currentNodeId = clickedNodeId;
      showContextMenu(event.clientX, event.clientY);
    } else {
      hideContextMenu();
    }
  });

  // Function to find a node at the clicked position
  function getNodeAtPosition(x, y) {
    for (let node of nodesArray) {
      const pos = positions[node.id];
      if (x >= pos.x - 50 && x <= pos.x + 50 && y >= pos.y - 25 && y <= pos.y + 25) {
        return node.id;
      }
    }
    return null;
  }

  // Show the context menu at the position of the right-click
  function showContextMenu(x, y) {
    const menu = document.getElementById('context-menu');
    menu.style.left = `${x}px`;
    menu.style.top = `${y}px`;
    menu.style.display = 'block';
  }

  // Hide the context menu
  function hideContextMenu() {
    const menu = document.getElementById('context-menu');
    menu.style.display = 'none';
  }

  // Handle context menu item click
  function handleContextMenuClick(option) {
    if (currentNodeId !== null) {
      const nodeName = nodesArray.find(node => node.id === currentNodeId).name;
      console.log(`Node: ${nodeName}, Option: ${option}`);
    }
    hideContextMenu();
  }


  let zoomFactor = 1.0; // Initial zoom factor
  let currentNodeId = null; // Currently selected node for context menu
  gitData = await fetchGitData();

  // Convert nodes from an object to an array for easier processing
  const nodesArray = Object.values(gitData.nodes);
  const edges = [];
  nodesArray.forEach(node => {
    node.children.forEach(childId => {
      edges.push({ from: node.id, to: childId });
    });
  });

  const positions = calculateNodePositions(nodesArray, edges); // Positions of nodes, calculated once and updated on zoom
  drawGraph(nodesArray, edges, positions);
});

// Adjust canvas and redraw graph on window resize
window.onresize = function () {
  const canvas = document.getElementById('canvas');
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  updateGraph();
};
