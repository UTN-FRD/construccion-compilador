import React from 'react';
import Tree from 'react-d3-tree';

const TreeView = () => {
  // Original JSON
  // { "List": [{"Atom": {"Id": "+"}}, {"Atom":{"Number": 1.0}}, {"Atom": {"Number": 2.0}}] }

  const mockedJson = {
    name: "List",
    children: [
      {
        name: "Atom",
        attributes: {
          type: "Id",
          value: "+",
          asd: "2"
        },
        children: [
          {
            name: "Atom",
            attributes: {
              type: "Id",
              value: "+",
              asd: "2"
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
        ]
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
