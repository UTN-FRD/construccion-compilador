import React from 'react';
import Tree from 'react-d3-tree';

const TreeView = ({ data }) => {
  return (
    <Tree data={data} orientation="vertical" />
  )
}

export default TreeView
