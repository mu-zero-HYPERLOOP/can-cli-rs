import argparse
import os
import pandas as pd
import matplotlib.pyplot as plt

def load_data(file_path):
    """ Load data from a CSV file. """
    try:
        return pd.read_csv(file_path, delimiter=';')
    except FileNotFoundError:
        print(f"Error: File not found {file_path}")
        return None
    except Exception as e:
        print(f"An error occurred while reading the file: {e}")
        return None

def parse_groups(groups):
    """ Parse groups from the command line arguments. """
    separate_groups = groups.split('|')  # Separate groups to be plotted on different figures
    parsed_groups = [group.split('&') for group in separate_groups]  # Entries to be plotted together
    return parsed_groups

def plot_data(data, node, object_entry, ax, separate_plots, color='blue'):
    """ Plot data on the given axis. """
    if separate_plots:
        plt.figure()
        ax = plt.gca()
    if len(data.columns) == 2:
        ax.plot(data.iloc[:, 0], data.iloc[:, 1], marker='o', linestyle='-', label=f"{node} {object_entry}", color=color)
    elif len(data.columns) > 2:
        fig, axes = plt.subplots(nrows=len(data.columns)-1, figsize=(10, 5 * (len(data.columns)-1)))
        for idx, column in enumerate(data.columns[1:]):
            current_ax = axes[idx] if len(data.columns) - 1 > 1 else axes
            current_ax.plot(data.iloc[:, 0], data.iloc[:, idx+1], marker='o', linestyle='-', label=f"{column}", color=color)
            current_ax.set_xlabel("Timestamp [us]")
            current_ax.set_ylabel(f"{column} Value")
            current_ax.grid(True)
        plt.tight_layout()
        plt.figure()
        plt.title(f"Additional Window for {node} {object_entry}")
        plt.show()
    ax.legend()
    if separate_plots:
        plt.show()

def main(args):
    """ Main function to process inputs and plot data. """
    if args.group:
        groups = parse_groups(args.group)
        colors = args.colors.split(',') if args.colors else ['blue'] * len(groups)  # Default color blue
        for group, color in zip(groups, colors):
            fig, ax = plt.subplots(figsize=(10, 5))
            for entry in group:
                node, object_entry = entry.split(':')
                file_path = os.path.join(args.path, node, f"{object_entry}.csv")
                data = load_data(file_path)
                if data is not None:
                    plot_data(data, node, object_entry, ax, args.multiple, color)
            if not args.multiple:
                plt.show()
    else:
        fig, ax = plt.subplots(figsize=(10, 5))
        for i in range(0, len(args.nodes), 2):
            node = args.nodes[i]
            object_entry = args.nodes[i+1]
            file_path = os.path.join(args.path, node, f"{object_entry}.csv")
            data = load_data(file_path)
            if data is not None:
                plot_data(data, node, object_entry, ax, args.multiple)
        if not args.multiple:
            plt.show()

if __name__ == "__main__":
    # Setup argument parser
    parser = argparse.ArgumentParser(description="Plot object entry data from log files.")
    parser.add_argument('path', type=str, help='Path to the logging directory')
    parser.add_argument('nodes', nargs='*', help='List of node and object-entry names (e.g., node1 object1 node2 object2)')
    parser.add_argument('-m', '--multiple', action='store_true', help='Create separate plots for each object entry')
    parser.add_argument('--group', type=str, help='Complex expression of node and object-entry names grouped by "&" and separated by "|"')
    parser.add_argument('--colors', type=str, help='Comma-separated list of colors for each group of plots')

    args = parser.parse_args()
    main(args)