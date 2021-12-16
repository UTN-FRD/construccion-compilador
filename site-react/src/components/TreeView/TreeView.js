import React from 'react';
import Tree from 'react-d3-tree';

const TreeView = ({ data }) => {
  const test = [
    {
      "name": "f",
      "children": [
        {
          "name": "x",
          "children": [
            {
              "name": "+"
            },
            {
              "name": "x"
            },
            {
              "name": 4
            }
          ]
        }
      ]
    },
    // {
    //   "name": "",
    //   "children": [
    //     {
    //       "name": "+"
    //     },
    //     {
    //       "name": "x"
    //     },
    //     {
    //       "name": 4
    //     }
    //   ]
    // }
  ]
  return (
    <Tree data={test} orientation="vertical" />
  )
}

export default TreeView
