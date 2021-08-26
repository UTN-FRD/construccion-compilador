import React from 'react';
import Tree from 'react-d3-tree';

const TreeView = () => {
  const mockedJson = {
    name: "List",
    children: [
      {
        name: "Atom",
        attributes: {
          type: "Id",
          value: "+"
        }
      },
      {
        name: "Atom",
        attributes: {
          type: "Number",
          value: "1"
        }
      },
      {
        name: "Atom",
        attributes: {
          type: "Number",
          value: "2"
        }
      },
    ],
  };

  return (
    <Tree data={mockedJson} orientation="vertical" />
  )
}

export default TreeView
